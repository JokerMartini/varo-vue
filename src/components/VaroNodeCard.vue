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
        
        <div class="flex space-x-4">
            <UIcon name='i-lucide-box' class="size-10"/>
            <div>
                <h3 class="text-md font-semibold">{{ node.name }}</h3>
                <p class="text-xs text-(--ui-text-dimmed)">{{ node.category }}</p>
            </div>
        </div>
    
        <div  v-if="node.variants.length > 1">
            <USelect
                v-model="selectedVariantIndex"
                :items="node.variants"
                class="w-full"
                labelKey="name"
                valueKey="id"
                variant="subtle"
                placeholder="Select variant"
            >
                <template #item="{ item }">
                    <div class="w-full space-y-1">
                        <div class="flex flex-row gap-1 items-start w-full flex-nowrap">
                            <div class="flex-1">
                                <h3 class="text-md font-semibold">{{ item.name }}</h3>
                            </div>
                            <UBadge 
                                v-if="item.status" 
                                class="rounded-sm " 
                                
                                variant="subtle"
                                :style="{ backgroundColor: item.status.background, color: item.status.color }"
                                >
                                {{ item.status.name }}
                            </UBadge>
                            <UButton variant="subtle" color="neutral" icon="i-lucide-rocket" size="xs"></UButton>                     
                        </div>
                        <p v-if="item.description" class="text-xs text-(--ui-text-dimmed)">{{ item.description }}</p>
                    </div>
                </template>

                <!-- <template #selected="{ option }">
                    <div class="flex flex-col text-left">
                    <span class="font-semibold text-sm">{{ option.title }}</span>
                    <span class="text-xs text-gray-500">{{ option.subtitle }}</span>
                    </div>
                </template> -->
            </USelect>

        </div>

        <!-- footer -->
        <!-- <UBadge 
            class="rounded-sm" 
            size="sm"
            color="neutral"
            variant="subtle"
            >
            {{ node.category }}
        </UBadge>   -->

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
