import { defineStore } from 'pinia'
import { VaroNode } from '~/models/VaroNode'
import { VaroNodeGroup } from '~/models/VaroNodeGroup'
import { VaroCategory } from '~/models/VaroCategory'
import { getVaroNodeGroups } from '~/utils/groupVaroNodes';
import { getVaroCategories } from '~/utils/groupVaroCategories';
import type { DisplayMode } from '~/types/DisplayMode';


export const useVaroNodeStore = defineStore('varoNodes', () => {
  const nodes = ref<VaroNode[]>([])
  const nodeGroups = ref<VaroNodeGroup[]>([])
  const displayMode = ref<DisplayMode>('ungrouped');

  function setNodes(newNodes: VaroNode[]) {
    nodes.value = newNodes
    nodeGroups.value = getVaroNodeGroups(newNodes)
  }

  const categories = computed<VaroCategory[]>(() =>
    getVaroCategories(nodes.value)
  );

  return {
    // properties
    nodes,
    nodeGroups,
    displayMode,

    // derived
    categories,

    // methods
    setNodes,
  }
})
