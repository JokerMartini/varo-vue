<script setup lang="ts">
import type { VaroNode } from '@/models/VaroNode';
import type { DropdownMenuItem } from '@nuxt/ui';

const toast = useToast()

const props = defineProps<{ node: VaroNode }>();

const items = ref<DropdownMenuItem[]>([
    {
        label: 'Hide',
        icon: 'i-lucide-eye-off',
        onSelect(e: Event) {
            props.node.visible = false
        }
    },
    {
        label: 'Unhide',
        icon: 'i-lucide-eye',
        onSelect(e: Event) {
            props.node.visible = true
        }
    },
    // {
    //     label: 'Edit',
    //     icon: 'i-lucide-pencil',
    //     onSelect(e: Event) {
    //         console.log('TOOD')
    //     }
    // }
])

function handleExecuteClick() {
  console.log('Button clicked!');
  toast.add({
    title: `Launching ${props.node.name}`,
    description: 'Your wish is my command...',
    icon: "i-lucide-rocket",
    color: "success"
  })
}

</script>

<template>
    <UContextMenu
        :items="items"
        :ui="{
            content: 'w-48'
        }"
    >
        <div class="bg-(--ui-bg-elevated)/50 p-2.5 rounded-[calc(var(--ui-radius)*2)] relative overflow-hidden"
            :class="{
                'outline-dashed outline-(--ui-text-dimmed) outline-2': !node.visible
            }"
            >
            
            <div class="flex flex-col gap-2 items-center justify-stretch w-full flex-nowrap">
                <!-- Action Button/Icon -->
                <UButton 
                    @click="handleExecuteClick"
                    icon="i-lucide-box"
                    variant="ghost" 
                    size="4xl" 
                    color="neutral" 
                    class="w-full justify-center shrink-0 cursor-pointer transition transform hover:scale-105 active:scale-90 duration-100 ease-out">
                </UButton>

                <div class="flex gap-2 w-full">
                    <!-- Title -->
                    <div class="flex flex-col w-full space-y-1 items-start">
                        <UTooltip :text="node.description" :disabled="!node.description">
                            <h3 class="text-sm font-semibold text-wrap">{{ node.name }}</h3>
                        </UTooltip>
        
                        <!-- Badges -->
                        <div class="flex gap-1">
                            <UBadge v-if="node.status" class="rounded-sm" size="sm" :color="node.status.color" variant="subtle">
                                {{ node.status.name }}
                            </UBadge>
                        </div>
                    </div>
                </div>
            </div>

        </div>
    </UContextMenu>
</template>


<!-- card horizontal / vertical -->