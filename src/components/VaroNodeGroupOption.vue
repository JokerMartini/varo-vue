<script setup lang="ts">
import type { VaroNode } from '@/models/VaroNode';

const toast = useToast()
const nodeStore = useVaroNodeStore()
const props = defineProps<{ node: VaroNode }>();
const emit = defineEmits<{
  (e: 'execute', node: VaroNode): void
}>()

function handleExecuteClick(event: Event) {
  event.stopImmediatePropagation();
  
  nodeStore.executeVaroNode(props.node)
  emit('execute', props.node)
}

</script>

<template>
    <div class="pr-1.5 relative overflow-hidden min-w-24 max-w-96">
        
        <div class="flex gap-2 items-stretch justify-stretch w-full flex-nowrap ">
            <!-- Action Button/Icon -->
            <div class="">
                <UTooltip text="Quick launch">
                    <UButton 
                        @click="handleExecuteClick"
                        icon="i-lucide-box" 
                        variant="ghost" 
                        size="2xl" 
                        color="neutral" 
                        class="shrink-0 cursor-pointer transition transform hover:scale-105 active:scale-90 duration-100 ease-out">
                    </UButton>
                </UTooltip>
            </div>

            <!-- Title -->
            <div class="flex flex-col space-y-1 items-start text-left">
                <UTooltip :text="node.description" :disabled="!node.description">
                    <h3 class="text-sm font-semibold">{{ node.name }}</h3>
                </UTooltip>

                <!-- Badges -->
                <div class="flex gap-1">
                    <UBadge v-if="node.status" class="rounded-sm" size="sm" :color="node.status.color" variant="subtle">
                        {{ node.status.name }}
                    </UBadge>
                </div>

                <!-- Description -->
                <p v-if="node.description" class="text-xs text-left text-(--ui-text-muted) text-wrap">
                    {{ node.description }}
                </p>
            </div>
        </div>

    </div>
</template>


<!-- card horizontal / vertical -->