<script setup lang="ts">
import { dummyVaroNodes } from '@/data/dummyNodes';
import type { ContextMenuItem } from '@nuxt/ui'

const nodeStore = useVaroNodeStore()

onMounted(() => {
  nodeStore.setNodes(dummyVaroNodes);
  nodeStore.loadFiles();
})

const mainMenuItems = computed(() => [
  [
    {
        label: "Toggle Groups",
        type: 'checkbox' as const,
        icon: "i-lucide-layout-list",
        checked: nodeStore.showGroups,
        onSelect(e: Event) {
            nodeStore.toggleGroups()
        },
    },
    {
        label: "Toggle Categories",
        type: 'checkbox' as const,
        icon: "i-lucide-shapes",
        checked: nodeStore.showCategories,
        onSelect(e: Event) {
            nodeStore.toggleCategories()
        },
    },
  ],
  [
    {
        label: "Unhide All Nodes",
        icon: "i-lucide-scan-eye",
        onSelect(e: Event) {
            nodeStore.unhideAllNodes();
            nodeStore.unhideAllNodeGroups();
        },
    },
    {
        label: "Toggle Hidden Nodes",
        type: 'checkbox' as const,
        checked: nodeStore.showHiddenNodes,
        icon: nodeStore.showHiddenNodes ? "i-lucide-eye" : "i-lucide-eye-off",
        onSelect(e: Event) {
            nodeStore.toggleHiddenNodeVisibility();
        },
    },
  ],
])
</script>

<template>
  <div class="h-dvh flex flex-col">
    <AppHeader/>

    <UContextMenu
      :items="mainMenuItems"
        :ui="{
          content: 'w-48'
        }"
      >
      <div class="p-3 grow">
        <CategorizedNodeListView v-if="nodeStore.showCategories"/>
        <template v-else>
          <NodeListView v-if="nodeStore.showGroups" v-model="nodeStore.filteredNodeGroups"/>
          <NodeListView v-else v-model="nodeStore.filteredNodes"/>
        </template>
      </div>
    </UContextMenu>

    <AppFooter/>

    <!-- Dialogs -->
    <AboutModal v-model="nodeStore.showAboutDialog" />
    <EnvModal v-model="nodeStore.showEnvDialog" />
    
  </div>
</template>