<script setup lang="ts">

const nodeStore = useVaroNodeStore();

</script>
<template>
    <div>
        <UAccordion :items="nodeStore.filteredCategories" type="multiple">
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
                <NodeListView v-if="nodeStore.showGroups" v-model="item.groups"/>
                <NodeListView v-else v-model="item.nodes"/>
              </div>
            </template>
        </UAccordion>
    </div>
</template>
