<script setup lang="ts">
const appStore = useAppStore()

onMounted(() => {
  appStore.initializeApp();
})

const mainMenuItems = computed(() => [
  [
    {
        label: "Toggle Groups",
        type: 'checkbox' as const,
        icon: "i-lucide-layout-list",
        checked: appStore.showGroups,
        onSelect(e: Event) {
            appStore.toggleGroups()
        },
    },
    {
        label: "Toggle Categories",
        type: 'checkbox' as const,
        icon: "i-lucide-shapes",
        checked: appStore.showCategories,
        onSelect(e: Event) {
            appStore.toggleCategories()
        },
    },
  ],
  [
    {
        label: "Refresh Data",
        icon: "i-lucide-refresh-cw",
        onSelect(e: Event) {
            appStore.refreshData();
        },
    },
    {
        label: "Unhide All Nodes",
        icon: "i-lucide-scan-eye",
        onSelect(e: Event) {
            appStore.unhideAllNodes();
            appStore.unhideAllNodeGroups();
        },
    },
    {
        label: "Toggle Hidden Nodes",
        type: 'checkbox' as const,
        checked: appStore.showHiddenNodes,
        icon: appStore.showHiddenNodes ? "i-lucide-eye" : "i-lucide-eye-off",
        onSelect(e: Event) {
            appStore.toggleHiddenNodeVisibility();
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
        <div v-if="appStore.loading" class="flex items-center justify-center h-32">
          <UIcon name="i-lucide-loader-2" class="animate-spin h-6 w-6" />
          <span class="ml-2">Loading...</span>
        </div>
        <template v-else>
          <CategorizedNodeListView v-if="appStore.showCategories"/>
          <template v-else>
            <NodeListView v-if="appStore.showGroups" v-model="appStore.filteredNodeGroups"/>
            <NodeListView v-else v-model="appStore.filteredNodes"/>
          </template>
        </template>
      </div>
    </UContextMenu>

    <AppFooter/>

    <!-- Dialogs -->
    <AboutModal v-model="appStore.showAboutDialog" />
    <DeveloperModal v-model="appStore.showDeveloperDialog" />
    
  </div>
</template>