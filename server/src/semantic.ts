//* LSP imports
import type { Diagnostic } from 'vscode-languageserver/node';
import { DiagnosticSeverity } from 'vscode-languageserver/node';

//* Parser imports
import type { DeclarationNode, StatementNode } from './parser';

//* Server imports
import { KNOWN_TYPES } from './knownSymbols';

export function checkUnknownTypes(statements: StatementNode[], diagnostics: Diagnostic[]): void {
  for (const stmt of statements) {
    if (stmt.type !== 'Declaration') continue;
    const decl = stmt as DeclarationNode;
    const typeName = decl.typeAnnotation.name;
    if (!KNOWN_TYPES.has(typeName)) {
      diagnostics.push({
        severity: DiagnosticSeverity.Warning,
        range: {
          start: { line: decl.typeAnnotation.range.start.line, character: decl.typeAnnotation.range.start.column },
          end: { line: decl.typeAnnotation.range.end.line, character: decl.typeAnnotation.range.end.column },
        },
        message: `Unknown type "${typeName}". Known types: ${[...KNOWN_TYPES].join(', ')}.`,
        source: 'hhat',
      });
    }
  }
}

