<script setup lang="ts">
import type { AccordionItem } from '@nuxt/ui'

const nodeStore = useVaroNodeStore();

const items = ref<AccordionItem[]>([
  {
    label: 'Icons',
    icon: 'i-lucide-layout-grid',
    content: 'You have nothing to do, @nuxt/icon will handle it automatically.'
  },
  {
    label: 'Colors',
    icon: 'i-lucide-layout-grid',
    content: 'Choose a primary and a neutral color from your Tailwind CSS theme.'
  },
  {
    label: 'Components',
    icon: 'i-lucide-layout-grid',
    content: 'You can customize components by using the `class` / `ui` props or in your app.config.ts.'
  }
])

</script>
<template>
    <div>

        <UAccordion :items="items" type="multiple"/>

      <!-- grouped categories -->
      <!-- <UAccordion :items="nodeStore.categories" type="multiple" :open="true">

        <template #leading="{ item }">
          <div class="flex items-center flex-1 justify-between">
              <h3 class="text-sm font-semibold text-wrap">{{ item.name }}</h3>
          </div>
        </template>

        <template #content="{ item }">
          <div class="pb-4">
              <div class="grid gap-4 grid-cols-[repeat(auto-fill,minmax(200px,1fr))]">
                <VaroNodeGroupCard
                  v-for="group in item.groups"
                  :key="group.id"
                  :group="group"
                />
              </div>
              <div class="grid gap-4 grid-cols-[repeat(auto-fill,minmax(200px,1fr))]">
                <VaroNodeCard
                  v-for="node in item.nodes"
                  :key="node.id"
                  :node="node"
                />
              </div>
          </div>
        </template>
      </UAccordion> -->

        <!-- <div class="grid gap-4 grid-cols-[repeat(auto-fill,minmax(180px,1fr))]">
            <VaroNodeCard v-for="node in nodeStore.filteredNodes" :key="node.id" :node="node" />
        </div>

        <USeparator class="py-6" />

        <div class="grid gap-4 grid-cols-[repeat(auto-fill,minmax(180px,1fr))]">
            <VaroNodeGroupCard v-for="group in nodeStore.filteredNodeGroups" :key="group.id" :group="group" />
        </div> -->
    </div>
</template>
