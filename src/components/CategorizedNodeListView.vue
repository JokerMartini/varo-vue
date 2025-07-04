<script setup lang="ts">

const appStore = useAppStore()

const filteredDisplayCategories = computed(() => {
  return appStore.filteredCategories.filter(category => {
    if (appStore.showGroups) {
      return category.groups.some(group => group.nodes.length > 0)
    } else {
      return category.nodes.length > 0
    }
  })
})

</script>
<template>
    <div>
        <UAccordion :items="filteredDisplayCategories" type="multiple">
            <!-- title -->
            <template #leading="{ item }">
                <div class="flex items-center flex-1 w-full  gap-1.5 font-medium text-sm">
                  <UIcon name="i-lucide-layout-grid" class="shrink-0 size-5"/>
                  <h3 class="text-sm font-semibold text-wrap">{{ item.name }}</h3>
                </div>
            </template>

            <!-- content -->
            <template #content="{ item }">
              <div class="pb-4">
                <NodeListView v-if="appStore.showGroups" v-model="item.groups"/>
                <NodeListView v-else v-model="item.nodes"/>
              </div>
            </template>
        </UAccordion>
    </div>
</template>