<script setup lang="ts">
import { dummyVaroNodes } from '@/data/dummyNodes';
import type { AccordionItem } from '@nuxt/ui';

const nodeStore = useVaroNodeStore()

onMounted(() => {
  nodeStore.setNodes(dummyVaroNodes);
})

const items = ref<AccordionItem[]>([
  {
    label: 'Icons',
    icon: 'i-lucide-smile',
    content: 'You have nothing to do, @nuxt/icon will handle it automatically.'
  },
  {
    label: 'Colors',
    icon: 'i-lucide-swatch-book',
    content: 'Choose a primary and a neutral color from your Tailwind CSS theme.'
  },
  {
    label: 'Components',
    icon: 'i-lucide-box',
    content: 'You can customize components by using the `class` / `ui` props or in your app.config.ts.'
  }
])

</script>

<template>
  <div class="h-dvh flex flex-col">
    <AppHeader/>
    <div class="p-4 grow">

      <!-- Ungrouped -->
      <p class="text-xs py-2 text-(--ui-text-muted)">
        Results ({{ nodeStore.nodes.length }} of {{ nodeStore.hiddenUngroupedCount }})
      </p>

      <div class="grid gap-4 grid-cols-[repeat(auto-fill,minmax(200px,1fr))]" >
        <VaroNodeCard
        v-for="node in nodeStore.nodes"
        :key="node.id"
        :node="node"
        />
      </div>
      
      <USeparator class="py-6"/>
      
      <!-- Grouped -->
      <p class="text-xs py-2 text-(--ui-text-muted)">
        Results ({{ nodeStore.nodeGroups.length }} of {{ nodeStore.hiddenGroupedCount }})
      </p>
      
      <div class="grid gap-4 grid-cols-[repeat(auto-fill,minmax(200px,1fr))]" >
        <VaroNodeGroupCard
        v-for="group in nodeStore.nodeGroups"
        :key="group.id"
        :group="group"
        />
      </div>

      <USeparator class="py-6"/>

      <!-- grouped categories -->
      <UAccordion :items="nodeStore.categories" type="multiple" :open="true">

        <template #leading="{ item }">
          <div class="flex items-center flex-1 justify-between">
              <h3 class="text-sm font-semibold text-wrap">{{ item.name }}</h3>
              <!-- <p class="text-xs py-2 text-(--ui-text-muted)">
                ({{ nodeStore.nodeGroups.length }} of {{ nodeStore.hiddenGroupedCount }})
              </p> -->
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
          </div>
        </template>
      </UAccordion>

    </div>
    <AppFooter/>
  </div>
</template>