<script setup lang="ts">
import { invoke } from '@tauri-apps/api/core';
import { ref } from 'vue';

const result = ref('');
const code = ref('');

async function execute() {
  result.value = await invoke('execute', { code: code.value });
}
</script>

<template>
  <main>
    <textarea v-model="code" cols="50" />
    <button @click="execute">
      Execute
    </button>
    <pre>{{ result }}</pre>
  </main>
</template>

<style scoped>
textarea {
  width: 100%;
  height: 40em;
  margin-bottom: 8px;
}
</style>
