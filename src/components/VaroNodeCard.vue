<script setup lang="ts">
import type { VaroNode } from '@/types/VaroNode';

const props = defineProps<{ node: VaroNode }>();

const items = ref<DropdownMenuItem[]>([
    {
        label: 'Hide',
        icon: 'i-lucide-eye-off'
    },
    {
        label: 'Edit',
        icon: 'i-lucide-pencil'
    }
])

</script>

<template>
    <div class="bg-(--ui-bg-elevated)/50 p-2.5 rounded-[calc(var(--ui-radius)*2)] relative overflow-hidden">

        <!-- hidden -->
        <div v-if="node.visible === false"
            class="w-12 h-12 -z-100 absolute top-0 left-0 w-full h-full text-(--ui-text-dimmed)/50
            bg-[size:10px_10px] 
            bg-fixed bg-[image:repeating-linear-gradient(315deg,currentColor_0,currentColor_1px,_transparent_0,_transparent_50%)]">
        </div>

        <div class="flex gap-2 items-start w-full flex-nowrap">
            <!-- Action Button/Icon -->
            <div>
                <UButton icon="i-lucide-box" variant="ghost" size="2xl" color="neutral" class="shrink-0 cursor-pointer">
                </UButton>
            </div>

            <!-- Title -->
            <div class="flex-grow space-y-1">
                <UTooltip :text="node.description" :disabled="!node.description">
                    <h3 class="text-md font-semibold">{{ node.name }}</h3>
                </UTooltip>

                <!-- Badges -->
                <div class="flex gap-1">
                    <UBadge v-if="node.status" class="rounded-sm" size="sm" :color="node.status.color" variant="subtle">
                        {{ node.status.name }}
                    </UBadge>
                </div>
            </div>

            <!-- Menu/Button -->
            <div>

                <UDropdownMenu size="md" :items="items" :content="{
                    align: 'start'
                }" :ui="{
                            content: 'w-48'
                        }">
                    <UButton icon="i-lucide-ellipsis-vertical" variant="link" color="neutral" class="shrink-0">
                    </UButton>
                </UDropdownMenu>
            </div>
        </div>

    </div>
</template>


<!-- card horizontal / vertical -->