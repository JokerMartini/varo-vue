<script setup lang="ts">
import type { VaroNode, VaroVariant } from '@/types/VaroNode';

const props = defineProps<{ node: VaroNode }>();

const variantOptions = computed(() =>
  props.node.variants.map((variant, index) => ({
    label: variant.name,
    value: index, // or use a unique ID if needed
  }))
);

// Ref to track selected variant
const selectedVariantIndex = ref<string | null>(null);

</script>

<template>
  <UCard class="w-full" variant="subtle">

    <div class="space-y-4 -m-2">
        
        <div class="flex items-center space-x-4">
                <UIcon name='i-lucide-box' class="size-10"/>
                <div>
                <h3 class="text-md font-semibold">{{ node.name }}</h3>
                <p class="text-xs text-(--ui-text-dimmed)">{{ node.category }}</p>
            </div>
        </div>
    
        <div>
            <USelect
            v-model="selectedVariantIndex"
            :items="node.variants"
            class="w-full"
            labelKey="name"
            valueKey="id"
            variant="subtle"
            placeholder="Select variant"
            />
        </div>

        <!-- <ul class="space-y-1">
            <li
            v-for="variant in node.variants"
            :key="variant.name"
            class="flex justify-between items-center text-sm"
            >
            <span>
                <strong>{{ variant.name }}</strong>
                <span v-if="variant.description" class="text-xs text-gray-500"> - {{ variant.description }}</span>
            </span>

            <span
                v-if="variant.status"
                class="text-xs font-medium px-2 py-1 rounded"
                :style="{ backgroundColor: variant.status.background, color: variant.status.color }"
            >
                {{ variant.status.name }}
            </span>
            </li>
        </ul> -->

    </div>

  </UCard>
</template>
