//* LSP imports
import { CompletionItemKind } from 'vscode-languageserver/node';

export interface KnownSymbol {
  label: string;
  detail: string;
  documentation: string;
  kind: CompletionItemKind;
}

export const KEYWORDS_AND_BUILTINS: KnownSymbol[] = [
  {
    label: 'main',
    detail: 'Entry block',
    documentation: 'Entry point. Defines the main block that runs when the program starts.\n\n```hhatq\nmain {\n  // ...\n}\n```',
    kind: CompletionItemKind.Keyword,
  },
  {
    label: 'fn',
    detail: 'Function definition',
    documentation: 'Defines a function.\n\n```hhatq\nfn add(a:i32 b:i32) i32 {\n  :: a\n}\n```',
    kind: CompletionItemKind.Keyword,
  },
  {
    label: 'metafn',
    detail: 'Meta-function definition',
    documentation: 'Defines a meta-function with named options.\n\n```hhatq\nmetafn measure(q:Qubit) Result {\n  basis: Z\n}\n```',
    kind: CompletionItemKind.Keyword,
  },
  {
    label: 'modifier',
    detail: 'Modifier definition',
    documentation: 'Defines a modifier that transforms a value.\n\n```hhatq\nmodifier double(self x:i32) i32 {\n  :: self\n}\n```',
    kind: CompletionItemKind.Keyword,
  },
  {
    label: 'super-type',
    detail: 'Super-type definition',
    documentation: 'Defines a super-type that groups related types.\n\n```hhatq\nsuper-type Number {\n  i32\n  f64\n}\n```',
    kind: CompletionItemKind.Keyword,
  },
  {
    label: 'type',
    detail: 'Type definition',
    documentation: 'Defines a struct or enum type.\n\n```hhatq\ntype Point {\n  x:f64\n  y:f64\n}\n```',
    kind: CompletionItemKind.Keyword,
  },
  {
    label: 'const',
    detail: 'Constant declaration',
    documentation: 'Declares a compile-time constant.\n\n```hhatq\nconst PI:f64 = 3.14159\n```',
    kind: CompletionItemKind.Keyword,
  },
  {
    label: 'use',
    detail: 'Import block',
    documentation: 'Imports functions, types, and other symbols.\n\n```hhatq\nuse (\n  fn: math.sqrt\n  type: physics.Particle\n)\n```',
    kind: CompletionItemKind.Keyword,
  },
  {
    label: 'true',
    detail: 'Boolean literal',
    documentation: 'Boolean true value.',
    kind: CompletionItemKind.Constant,
  },
  {
    label: 'false',
    detail: 'Boolean literal',
    documentation: 'Boolean false value.',
    kind: CompletionItemKind.Constant,
  },
  {
    label: 'self',
    detail: 'Self reference',
    documentation: 'References the receiver in a modifier definition.',
    kind: CompletionItemKind.Variable,
  },
  {
    label: 'print',
    detail: 'Built-in function',
    documentation: 'Prints a value to standard output.\n\n```hhatq\nprint(x)\n```',
    kind: CompletionItemKind.Function,
  },
];

export const TYPES: KnownSymbol[] = [
  { label: 'i8', detail: '8-bit signed integer', documentation: 'Signed 8-bit integer type.', kind: CompletionItemKind.TypeParameter },
  { label: 'i16', detail: '16-bit signed integer', documentation: 'Signed 16-bit integer type.', kind: CompletionItemKind.TypeParameter },
  { label: 'i32', detail: '32-bit signed integer', documentation: 'Signed 32-bit integer type.', kind: CompletionItemKind.TypeParameter },
  { label: 'i64', detail: '64-bit signed integer', documentation: 'Signed 64-bit integer type.', kind: CompletionItemKind.TypeParameter },
  { label: 'u8', detail: '8-bit unsigned integer', documentation: 'Unsigned 8-bit integer type.', kind: CompletionItemKind.TypeParameter },
  { label: 'u16', detail: '16-bit unsigned integer', documentation: 'Unsigned 16-bit integer type.', kind: CompletionItemKind.TypeParameter },
  { label: 'u32', detail: '32-bit unsigned integer', documentation: 'Unsigned 32-bit integer type.', kind: CompletionItemKind.TypeParameter },
  { label: 'u64', detail: '64-bit unsigned integer', documentation: 'Unsigned 64-bit integer type.', kind: CompletionItemKind.TypeParameter },
  { label: 'f32', detail: '32-bit float', documentation: '32-bit floating-point type.', kind: CompletionItemKind.TypeParameter },
  { label: 'f64', detail: '64-bit float', documentation: '64-bit floating-point type.', kind: CompletionItemKind.TypeParameter },
  { label: 'string', detail: 'String type', documentation: 'UTF-8 text value, written between double quotes. Example: `name: string = \"Ada\"`.', kind: CompletionItemKind.TypeParameter },
];

export const ALL_KNOWN: KnownSymbol[] = [...KEYWORDS_AND_BUILTINS, ...TYPES];

export const KNOWN_TYPES = new Set(TYPES.map((t) => t.label));
