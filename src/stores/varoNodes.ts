import { defineStore } from 'pinia'
import { VaroNode } from '~/models/VaroNode'
import { VaroNodeGroup } from '~/models/VaroNodeGroup'
import { VaroCategory } from '~/models/VaroCategory'
import { getVaroNodeGroups } from '~/utils/groupVaroNodes';
import { getVaroCategories } from '~/utils/groupVaroCategories';
import type { DisplayMode } from '~/types/DisplayMode';
import { invoke } from '@tauri-apps/api/core';


export const useVaroNodeStore = defineStore('varoNodes', () => {
  const showHiddenNodes = ref(false)
  const showGroups = ref(false)
  const showCategories = ref(false)
  const showAboutDialog = ref(false)
  const showEnvDialog = ref(false)
  const searchQuery = ref('')
  const allNodes = ref<VaroNode[]>([])
  const allNodeGroups = ref<VaroNodeGroup[]>([])
  const displayMode = ref<DisplayMode>('ungrouped');
  const loading = ref(false)
  const username = ref<string | null>(null)
  const platform = ref<string | null>(null)

  // METHODS
  function setNodes(newNodes: VaroNode[]) {
    allNodes.value = newNodes
    allNodeGroups.value = getVaroNodeGroups(newNodes)

    // for testing
    allNodeGroups.value[1].visible = false;
  }
  
  function toggleHiddenNodeVisibility() {
    showHiddenNodes.value = !showHiddenNodes.value
  }
  
  function toggleGroups() {
    showGroups.value = !showGroups.value
  }
  
  function toggleCategories() {
    showCategories.value = !showCategories.value
  }

  async function fetchUsername() {
    username.value = await invoke<string>('get_os_username')
    console.log(username.value);
  }

  async function fetchPlatform() {
    platform.value = await invoke<string>('get_platform')
    console.log(platform.value);
  }

  function executeVaroNode(node: VaroNode) {
    console.log("Node executing: >>>", node)
  }

  // COMPUTED PROPERTIES
  const categories = computed<VaroCategory[]>(() =>
    getVaroCategories(nodes.value)
  );

  const filteredNodes = computed(() => {
    const query = searchQuery.value.trim().toLowerCase()
  
    return allNodes.value.filter(node => {
      const matchesSearch = !query || 
        node.name.toLowerCase().includes(query) ||
        node.description?.toLowerCase().includes(query)
      const isVisible = showHiddenNodes.value || node.visible
      return matchesSearch && isVisible
    })
  })

  const filteredNodeGroups = computed(() => {
    const query = searchQuery.value.trim().toLowerCase()

    return allNodeGroups.value.filter(group => {
      // Check if the group contains at least one matching and visible node
      return group.nodes.some(node => {
        const matchesSearch = !query || node.name.toLowerCase().includes(query)
        const isVisible = showHiddenNodes.value || group.visible
        return matchesSearch && isVisible
      })
    })
  })









  async function fetchVaroNodes() {
    const result = await invoke<string>('get_varo_nodes')
    console.log(result.nodes)
    console.log(result.warnings)
  } 
  
 

  async function launchSomething() {
    const result = await invoke('execute_program', {
      path: 'C:/Windows/notepad.exe',
      args: [],
      wait: true
    })
    const result2 = await invoke('execute_program', {
      path: 'C:/Program Files/Python310/python.exe',
      args: ["C:/Users/joker/Documents/GitHub/varo-vue/test-data/nodes/scripts/simpleDialog.py"],
      wait: true
    })
    const result3 = await invoke('execute_program', {
      path: 'C:/Program Files/Python310/python.exe',
      args: ["C:/Users/joker/Documents/GitHub/varo-vue/test-data/nodes/scripts/simpleDialog.py"],
      wait: false,
      envVars: {
        AAAA: 'KEVIN',
        AAZZ: '123'
      }
    })
  }

  async function loadFiles() {
    await fetchUsername()
    await fetchPlatform()
    fetchVaroNodes();

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
    allNodes,
    filteredNodes,
    filteredNodeGroups,
    allNodeGroups,
    displayMode,
    loading,
    showHiddenNodes,
    showGroups,
    showCategories,
    showAboutDialog,
    showEnvDialog,
    searchQuery,
    username,
    platform,

    // derived
    categories,
    
    // methods
    setNodes,
    loadFiles,
    launchSomething,
    toggleHiddenNodeVisibility,
    toggleGroups,
    toggleCategories,
    executeVaroNode
  }
})