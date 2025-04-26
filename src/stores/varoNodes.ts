import { defineStore } from 'pinia'
import { VaroNode } from '~/models/VaroNode'
import { VaroNodeGroup } from '~/models/VaroNodeGroup'
import { VaroCategory } from '~/models/VaroCategory'
import { getVaroNodeGroups } from '~/utils/groupVaroNodes';
import { getVaroCategories } from '~/utils/groupVaroCategories';
import type { DisplayMode } from '~/types/DisplayMode';
import { invoke } from '@tauri-apps/api/core';


export const useVaroNodeStore = defineStore('varoNodes', () => {
  const nodes = ref<VaroNode[]>([])
  const nodeGroups = ref<VaroNodeGroup[]>([])
  const displayMode = ref<DisplayMode>('ungrouped');
  const loading = ref(false)

  function setNodes(newNodes: VaroNode[]) {
    nodes.value = newNodes
    nodeGroups.value = getVaroNodeGroups(newNodes)
  }

  const categories = computed<VaroCategory[]>(() =>
    getVaroCategories(nodes.value)
  );

  async function fetchUsername() {
    const username = await invoke<string>('get_os_username')
    console.log(`Welcome ${username}!`)
  }

  async function fetchVaroNodes() {
    const result = await invoke<string>('get_varo_nodes')
    console.log(result.nodes)
    console.log(result.warnings)
  } 
  
  async function fetchPlatform() {
    const platform = await invoke<string>('get_platform')
    console.log("Running on:", platform);
  }

  async function loadFiles() {
    fetchUsername();
    fetchVaroNodes();
    fetchPlatform();

    // loading.value = true
    // // error.value = null

    // try {
    //   const files = await invoke<{ filepath: string; content: string; modified: string }[]>(
    //     'read_json_files_from_varo_path'
    //   )
    //   console.log(files)
    //   files.forEach(f => {
    //     console.log(f)
    //   });
    //   // const varoFiles = files.map(file => new VaroFile(file))
    // } catch (err: any) {
    //   console.error('Failed to load files:', err)
    //   // error.value = err.message ?? 'Unknown error'
    // } finally {
    //   loading.value = false
    // }
  }

  return {
    // properties
    nodes,
    nodeGroups,
    displayMode,
    loading,

    // derived
    categories,

    // methods
    setNodes,
    loadFiles,
  }
})
