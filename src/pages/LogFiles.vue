<template>
  <div class="container mx-auto p-6">
    <h1 class="text-2xl font-bold mb-6">Log Files</h1>
    
    <div class="flex justify-between mb-6">
      <UButton 
        color="primary" 
        @click="fetchLogs"
        :loading="loading"
      >
        Refresh Files
      </UButton>

      <UButton color="primary" @click="goBack">Go Back</UButton>
    </div>
    
    <div v-if="error" class="text-red-500 mb-4 p-4 bg-red-100 rounded">
      {{ error }}
    </div>
    
    <div v-if="logs.length === 0 && !loading && !error" class="text-gray-500 italic">
      No log files found or haven't refreshed yet.
    </div>
    
    <div v-if="logs.length > 0" class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-6">
      <div v-for="(log, index) in logs" :key="index" class="file-card border rounded p-4 shadow-md">
        <h2 class="text-lg font-semibold">{{ log.path }}</h2>
        <p class="text-gray-600 italic">Last modified: {{ log.modified }}</p>
        <UButton @click="viewFileContent(log)">View Details</UButton>
      </div>
    </div>

    <!-- Modal for file content -->
    <div v-if="selectedFile" class="file-content-modal">
      <div class="modal-content">
        <button class="close-modal" @click="closeFileContent">×</button>
        <h2>{{ selectedFile.path }}</h2>
        <pre>{{ selectedFile.content }}</pre>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import { useRouter } from 'vue-router'; // Importing router for navigation
import { attachConsole } from '@tauri-apps/plugin-log';

const detach = await attachConsole();

interface LogFile {
  path: string;
  modified: string;
  content: string;
}

const logs = ref<LogFile[]>([]);
const loading = ref(false); // Add loading state
const error = ref<string | null>(null); // Add error state
const selectedFile = ref<LogFile | null>(null);
const router = useRouter(); // Initialize the router instance

console.log('✅ invoke loaded:', invoke);

const viewFileContent = async (log: LogFile) => {
  selectedFile.value = { ...log };
};

const closeFileContent = () => {
  selectedFile.value = null; // Clear the selected file
};

const fetchLogs = async () => {
  loading.value = true;
  error.value = null;
  try {
    console.log("Invoking Tauri command...");
    if (typeof invoke !== 'function') {
      console.error("invoke is undefined or not a function");
    } else {
      logs.value = await invoke('fetch_log_files'); 
      console.log("Logs:", logs.value);
    }
  } catch (err) {
    error.value = "Error fetching log files: " + err.message;
    console.error("Error fetching log files:", err);
  } finally {
    loading.value = false;
  }
};

const goBack = () => {
  router.push('/'); // Navigate back to the main page
};

onMounted(() => {
  fetchLogs(); // Fetch logs when the component is mounted
});
</script>

<style scoped>
.log-files {
  padding: 20px;
}
</style>
