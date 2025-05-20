<script setup lang="ts">
import type { VaroNode } from '@/models/VaroNode';
import type { DropdownMenuItem } from '@nuxt/ui';

const props = defineProps<{ node: VaroNode }>();
const nodeStore = useVaroNodeStore()
const nodeCard = useTemplateRef('nodeCard')

const menuItems = computed<DropdownMenuItem[][]>(() => [
  [
    {
      label: 'Hide',
      disabled: !props.node.visible,
      icon: 'i-lucide-eye-off',
      onSelect(e: Event) {
        props.node.visible = false;
      }
    },
    {
      label: 'Unhide',
      disabled: props.node.visible,
      icon: 'i-lucide-eye',
      onSelect(e: Event) {
        props.node.visible = true;
      }
    },
    {
      label: 'Unhide All',
      icon: 'i-lucide-eye',
      onSelect(e: Event) {
        nodeStore.unhideAllNodes()
      }
    }
  ],
  [
    {
      label: 'Show in Explorer...',
      icon: 'i-lucide-folder',
      onSelect(e: Event) {
        console.log('TODO');
      }
    }
  ]
]);

function handleExecuteClick() {
  nodeCard.value?.classList.add('animate-scale-bounce')
  setTimeout(() => {
    nodeCard.value?.classList.remove('animate-scale-bounce')
  }, 300)

  nodeStore.executeVaroNode(props.node)
}

</script>

<template>
    <UContextMenu
        :items="menuItems"
    >
        <div 
          ref="nodeCard"
          @click="handleExecuteClick"
          class="
          bg-(--ui-bg-elevated)/75 p-2.5 rounded-[calc(var(--ui-radius)*2)] relative overflow-hidden
          hover:bg-(--ui-bg-elevated)
          active:bg-(--ui-bg-elevated)/50
          cursor-pointer
          "
          :class="{
              'outline-dashed outline-(--ui-text-dimmed) outline-2': !node.visible
          }"
            >

            <div class="flex gap-3 items-stretch justify-stretch w-full flex-nowrap ">
                <!-- Action Button/Icon -->
                <div class="shrink-0">
                  <UIcon name="i-lucide-box" class="shrink-0 size-12"/>
                </div>

                <!-- Title -->
                <div class="flex flex-col w-full space-y-1 items-start">
                    <UTooltip :text="node.description" :disabled="!node.description">
                        <h3 class="text-xs font-semibold text-wrap">{{ node.name }}</h3>
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
    </UContextMenu>
</template>