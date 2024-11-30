import type { Plugin } from 'vue';
import { install } from '@guolao/vue-monaco-editor';
import * as Monaco from 'monaco-editor';
import EditorWorker from 'monaco-editor/esm/vs/editor/editor.worker?worker';
import TsWorker from 'monaco-editor/esm/vs/language/typescript/ts.worker?worker';

export function monaco(): Plugin {
  window.MonacoEnvironment = {
    getWorker(_, label) {
      if (label === 'typescript' || label === 'javascript')
        return new TsWorker();
      return new EditorWorker();
    },
  };

  const tsCompilerOptions: Parameters<typeof Monaco.languages.typescript.typescriptDefaults.setCompilerOptions>[0] = {
    strict: true,
    lib: ['ESNext'],
    allowNonTsExtensions: true,
  };
  Monaco.languages.typescript.typescriptDefaults.setCompilerOptions(tsCompilerOptions);
  Monaco.languages.typescript.javascriptDefaults.setCompilerOptions(tsCompilerOptions);

  return () => install({ monaco: Monaco });
}
