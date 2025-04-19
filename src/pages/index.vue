<script setup lang="ts">
import { dummyVaroNodes } from '@/data/dummyNodes';
import { getVaroNodeGroups } from '@/utils/groupVaroNodes';
import { invoke } from '@tauri-apps/api/core';
import { ref } from 'vue';

const nodeStore = useVaroNodeStore()

onMounted(() => {
  nodeStore.setNodes(dummyVaroNodes);
})

const items = ref<AccordionItem[]>([
  {
    label: 'Icons',
    icon: 'i-lucide-smile',
    content: 'You have nothing to do, @nuxt/icon will handle it automatically.'
  },
  {
    label: 'Colors',
    icon: 'i-lucide-swatch-book',
    content: 'Choose a primary and a neutral color from your Tailwind CSS theme.'
  },
  {
    label: 'Components',
    icon: 'i-lucide-box',
    content: 'You can customize components by using the `class` / `ui` props or in your app.config.ts.'
  }
])

interface TextFile {
  path: string;
  modified: string;
  content: string;
}

const textFiles = ref<TextFile[]>([]);
const loading = ref(false);
const error = ref('');

async function refreshTextFiles() {
  loading.value = true;
  error.value = '';
  textFiles.value = []; // Clear existing files
  
  try {
    const files = await invoke<TextFile[]>('get_text_files');
    textFiles.value = files;
  } catch (err) {
    error.value = err as string;
  } finally {
    loading.value = false;
  }
}
</script>

<template>
  <div class="container mx-auto p-6">
    <h1 class="text-2xl font-bold mb-6">VARO Text Files</h1>
    
    <UButton 
      color="primary" 
      @click="refreshTextFiles"
      class="mb-6"
      :loading="loading"
    >
      Refresh Files
    </UButton>
    
    <div v-if="error" class="text-red-500 mb-4 p-4 bg-red-100 rounded">
      {{ error }}
    </div>
    
    <div v-if="textFiles.length === 0 && !loading && !error" class="text-gray-500 italic">
      No text files found or haven't refreshed yet.
    </div>
    
    <ol v-else class="space-y-6 list-decimal pl-5">
      <li v-for="file in textFiles" :key="file.path" class="border rounded p-4">
        <h2 class="text-lg font-semibold">{{ file.path }}</h2>
        <p class="text-gray-600 italic">Last modified: {{ file.modified }}</p>
        <p class="mt-2 whitespace-pre-wrap">{{ file.content }}</p>
      </li>
    </ol>
  </div>
</template>