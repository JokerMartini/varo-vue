import { defineStore } from 'pinia'
import { getVaroNodeGroups } from '@/utils/groupVaroNodes'
import type { VaroNode } from '~/models/VaroNode'
import { VaroNodeGroup } from '~/models/VaroNodeGroup'

export const useVaroNodeStore = defineStore('varoNodes', () => {
  const nodes = ref<VaroNode[]>([])
  const nodeGroups = ref<VaroNodeGroup[]>([])

  function setNodes(newNodes: VaroNode[]) {
    nodes.value = newNodes
    nodeGroups.value = getVaroNodeGroups(newNodes)
  }

  return {
    // properties
    nodes,
    nodeGroups,
    // methods
    setNodes,
  }
})
