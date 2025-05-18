<script setup lang="ts">
import { dummyVaroNodes } from '@/data/dummyNodes';

const nodeStore = useVaroNodeStore()

onMounted(() => {
  nodeStore.setNodes(dummyVaroNodes);
  nodeStore.loadFiles();
})

</script>

<template>
  <div class="h-dvh flex flex-col">
    <AppHeader/>
    <div class="p-4 grow">

      <!-- list -->
      <NodeListView v-if="nodeStore.showGroups" v-model="nodeStore.filteredNodeGroups"/>
      <NodeListView v-else v-model="nodeStore.filteredNodes"/>
      
      <!-- <CategorizedNodeListView/> -->

    </div>
    <AppFooter/>

    <!-- Dialogs -->
    <AboutModal v-model="nodeStore.showAboutDialog" />
    <EnvModal v-model="nodeStore.showEnvDialog" />
    
  </div>
</template>