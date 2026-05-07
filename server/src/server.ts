//* LSP imports
import {
  createConnection,
  TextDocuments,
  ProposedFeatures,
  type InitializeParams,
  type InitializeResult,
  TextDocumentSyncKind,
  type Diagnostic,
  DiagnosticSeverity,
  type CompletionItem,
  type Hover,
  MarkupKind,
} from 'vscode-languageserver/node';

//* TextDocument imports
import { TextDocument } from 'vscode-languageserver-textdocument';

//* Parser imports
import { parse } from './parser';
import type { ParseDiagnostic } from './parser';

//* Server imports
import { ALL_KNOWN, TYPES } from './knownSymbols';
import { checkUnknownTypes } from './semantic';

const connection = createConnection(ProposedFeatures.all);
const documents = new TextDocuments(TextDocument);

function collectDeclaredVariableTypes(text: string): Map<string, string> {
  const declared = new Map<string, string>();
  const declarationWithType = /\b([a-zA-Z_][a-zA-Z0-9_]*)\s*:\s*([a-zA-Z_][a-zA-Z0-9_]*)/g;

  let match: RegExpExecArray | null;
  while ((match = declarationWithType.exec(text)) !== null) {
    const variableName = match[1];
    const typeName = match[2];
    if (variableName && typeName) {
      declared.set(variableName, typeName);
    }
  }

  return declared;
}

connection.onInitialize((params: InitializeParams): InitializeResult => {
  const result: InitializeResult = {
    capabilities: {
      textDocumentSync: TextDocumentSyncKind.Incremental,
      completionProvider: {
        resolveProvider: false,
      },
      hoverProvider: true,
    },
  };

  connection.console.info(`Initialized for ${params.clientInfo?.name ?? 'unknown client'}.`);
  return result;
});

documents.onDidChangeContent((change) => {
  validateTextDocument(change.document);
});

documents.onDidOpen((event) => {
  validateTextDocument(event.document);
});

connection.onCompletion((): CompletionItem[] => {
  return ALL_KNOWN.map((item) => {
    const completionItem: CompletionItem = {
      label: item.label,
      kind: item.kind,
      detail: item.detail,
      documentation: {
        kind: MarkupKind.Markdown,
        value: item.documentation,
      },
    };

    return completionItem;
  });
});

connection.onHover((params): Hover | null => {
  const document = documents.get(params.textDocument.uri);
  if (!document) {
    return null;
  }

  const offset = document.offsetAt(params.position);
  const text = document.getText();

  const word = getWordAt(text, offset);
  if (!word) {
    return null;
  }

  const known = ALL_KNOWN.find((k) => k.label === word);
  if (known) {
    return {
      contents: {
        kind: MarkupKind.Markdown,
        value: `**${known.label}**\n\n${known.documentation}\n`,
      },
    };
  }

  const declaredVariableTypes = collectDeclaredVariableTypes(text);
  const declaredType = declaredVariableTypes.get(word);
  if (!declaredType) {
    return null;
  }

  const knownType = TYPES.find((t) => t.label === declaredType);
  const typeDoc = knownType ? `\n\n${knownType.documentation}\n` : '\n';

  return {
    contents: {
      kind: MarkupKind.Markdown,
      value: `**${word}**: \`${declaredType}\`${typeDoc}`,
    },
  };
});

const LSP_SEVERITY_MAP: Record<ParseDiagnostic['severity'], DiagnosticSeverity> = {
  error: DiagnosticSeverity.Error,
  warning: DiagnosticSeverity.Warning,
  info: DiagnosticSeverity.Information,
};

function validateTextDocument(textDocument: TextDocument): void {
  const text = textDocument.getText();
  const result = parse(text);
  const diagnostics: Diagnostic[] = [];

  for (const d of result.diagnostics) {
    diagnostics.push({
      severity: LSP_SEVERITY_MAP[d.severity],
      range: {
        start: { line: d.range.start.line, character: d.range.start.column },
        end: { line: d.range.end.line, character: d.range.end.column },
      },
      message: d.message,
      source: 'hhat',
    });
  }

  // Semantic check: unknown types on declarations (grammar does not enforce type validity)
  if (result.ast?.main) {
    checkUnknownTypes(result.ast.main.body, diagnostics);
  }
  for (const def of result.ast?.definitions ?? []) {
    if (def.type === 'FunctionDef') {
      checkUnknownTypes(def.body, diagnostics);
    }
  }

  connection.sendDiagnostics({ uri: textDocument.uri, diagnostics });
}

function getWordAt(text: string, offset: number): string | null {
  if (offset < 0 || offset > text.length) {
    return null;
  }

  const isWordChar = (char: string) => /[A-Za-z0-9_]/.test(char);

  let start = offset;
  while (start > 0 && isWordChar(text[start - 1] ?? '')) {
    start -= 1;
  }

  let end = offset;
  while (end < text.length && isWordChar(text[end] ?? '')) {
    end += 1;
  }

  const word = text.slice(start, end);
  return word.length > 0 ? word : null;
}

documents.listen(connection);
connection.listen();
