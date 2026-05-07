//* Libraries imports
import { describe, expect, it } from 'vitest';

//* LSP imports
import type { Diagnostic } from 'vscode-languageserver/node';

//* Parser imports
import { parse } from '../parser';

//* Server imports
import { checkUnknownTypes } from '../semantic';

describe('semantic type validation', () => {
  it('does not warn for built-in string type', () => {
    const source = 'main { name:string = "Ada" }';
    const result = parse(source);

    const diagnostics: Diagnostic[] = [];
    checkUnknownTypes(result.ast?.main?.body ?? [], diagnostics);

    expect(diagnostics).toEqual([]);
  });

  it('warns for unknown type names', () => {
    const source = 'main { x:MadeUpType = 1 }';
    const result = parse(source);

    const diagnostics: Diagnostic[] = [];
    checkUnknownTypes(result.ast?.main?.body ?? [], diagnostics);

    expect(diagnostics.length).toBe(1);
    expect(diagnostics[0]?.message).toContain('Unknown type "MadeUpType"');
  });
});

