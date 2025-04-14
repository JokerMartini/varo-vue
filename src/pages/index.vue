<script setup lang="ts">
import { dummyVaroNodes } from '@/data/dummyNodes';
import { getVaroNodeGroups } from '@/utils/groupVaroNodes';

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

const varoNodeGroups = getVaroNodeGroups(dummyVaroNodes);

</script>

<template>
  <div class="p-4">

    <!-- Ungrouped -->
    <p class="font-semibold text-lg py-2">Ungrouped {{dummyVaroNodes.length}} nodes</p>

    <div class="grid gap-4 grid-cols-[repeat(auto-fill,minmax(200px,1fr))]">
      <VaroNodeCard
      v-for="node in dummyVaroNodes"
      :key="node.uid"
      :node="node"
      />
    </div>
    
    <USeparator class="py-6"/>
    
    <!-- Grouped -->
    <p class="font-semibold text-lg py-2">Grouped {{varoNodeGroups.length}} nodes</p>
    
    <div class="grid gap-4 grid-cols-[repeat(auto-fill,minmax(200px,1fr))]">
      <VaroNodeGroupCard
      v-for="group in varoNodeGroups"
      :key="group.id"
      :group="group"
      />
    </div>

    <!-- grouped categories -->
    <!-- <UAccordion :items="items" type="multiple" /> -->
  </div>
</template>