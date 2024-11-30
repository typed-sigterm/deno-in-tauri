<script setup lang="ts">
import { VueMonacoEditor } from '@guolao/vue-monaco-editor';
import { IconDebugStart, IconGithub } from '@iconify-prerendered/vue-codicon';
import { invoke } from '@tauri-apps/api/core';
import swal from 'sweetalert';
import { ref, watch } from 'vue';
import IconLoading from './IconLoading.vue';
import '@vscode-elements/elements/dist/vscode-button/index.js';

const code = ref(localStorage.getItem('code') ?? `
export default () => {
  return 114514 * 1919810;
}
`.trim());

const executing = ref(false);
const saved = ref(true);

async function execute() {
  // Don't show the spinner if the execution is too fast
  const timeout = setTimeout(() => executing.value = true, 50);
  try {
    const result = await invoke<string>('execute', { code: code.value });
    swal({
      icon: 'success',
      text: result,
    });
  } catch (err) {
    swal({
      icon: 'error',
      text: String(err),
    });
  }
  clearTimeout(timeout);
  if (executing.value)
    executing.value = false;
}

function save() {
  localStorage.setItem('code', code.value);
  saved.value = true;
}

watch(code, () => saved.value = false);
</script>

<template>
  <div class="editor">
    <VueMonacoEditor
      v-model:value="code"
      theme="vs"
      language="javascript"
      :options="{
        automaticLayout: true,
      }"
    />
  </div>

  <div class="operations">
    <vscode-button :disabled="executing" @click="execute">
      <IconLoading v-if="executing" width="1em" height="1em" />
      <IconDebugStart v-else />
      Execute
    </vscode-button>
    <vscode-button secondary :disabled="saved" @click="save">
      Save
    </vscode-button>
  </div>

  <a class="github" href="https://github.com/typed-sigterm/deno-in-tauri" target="_blank">
    <IconGithub width="24px" height="24px" alt="GitHub" />
  </a>
</template>

<style>
body {
  overflow: hidden;
  user-select: none;
}

#app {
  display: flex;
  flex-direction: column;
  height: calc(100vh - 16px - 4px);
}
</style>

<style scoped>
.editor {
  flex: 1;
  margin-bottom: 4px;
}

.operations {
  display: flex;
  justify-content: center;
  gap: 8px;
}

.github {
  display: block;
  position: fixed;
  right: 12px;
  bottom: 12px;
  color: initial;
}
</style>

<style src="./theme.css" />
