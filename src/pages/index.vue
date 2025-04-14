<script setup lang="ts">
import { dummyVaroNodes } from '@/data/dummyNodes';

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
  <div class="p-4">

    <div class="grid gap-4 grid-cols-[repeat(auto-fill,minmax(220px,1fr))]">
      <VaroNodeCard
        v-for="node in dummyVaroNodes"
        :key="node.uid"
        :node="node"
      />
    </div>

    <!-- grouped categories -->
    <UAccordion :items="items" type="multiple" />
  </div>
</template>