<script lang='ts' setup>
import Select from 'primevue/select'
import Textarea from 'primevue/textarea'
import { computed, ref, toRaw, watch } from 'vue'
import type { IEndpointConfigItem } from '../../database/endpointConfig'
import type { IPromptConfigItem } from '../../database/promptConfig'
import EditableInputText from '../EditableInputText.vue'
import Icon from '../Icon.vue'
import IconPicker from '../IconPicker.vue'

export interface PromptItemSettingProps {
  item: IPromptConfigItem
  endpoints?: IEndpointConfigItem[]
}

const props = defineProps<PromptItemSettingProps>()
const emit = defineEmits(['remove', 'update'])

const editMode = ref(!props.item.id)

const currentValue = ref(structuredClone(toRaw(props.item)))

watch(
  () => props.item,
  () => {
    currentValue.value = structuredClone(toRaw(props.item))
  },
)

const endpoints = computed(() => props.endpoints ?? [])

function toggleEditMode() {
  editMode.value = !editMode.value
  currentValue.value = structuredClone(toRaw(props.item))

  if (!props.item.id) {
    emit('remove')
  }
}

function applyUpdate() {
  emit('update', currentValue.value)
  editMode.value = !editMode.value
}
</script>

<template>
  <div class="flex flex-col gap-2 bg-light-3 p-4 rounded">
    <div class="flex items-center gap-1">
      <IconPicker v-model="currentValue.icon" />
      <EditableInputText :editable="editMode" v-model="currentValue.name" />
      <div class="flex flex-1 justify-end">
        <Icon v-if="!currentValue.isBuiltin" class="i-carbon:close cursor-pointer" @click="emit('remove')" />
      </div>
    </div>
    <div class="flex flex-col gap-2">
      <Textarea v-model="currentValue.prompt" :rows="5" />
    </div>
    <div class="editable-row">
      <label>Endpoint</label>
      <div class="flex gap-2">
        <Select v-model="currentValue.endpointId" :options="endpoints" optionLabel="name" optionValue="id"
          placeholder="Select a endpoint" />
      </div>
    </div>
    <div class="flex items-center gap-2 justify-end mt-2">
      <template v-if="editMode">
        <Icon class="i-carbon:close cursor-pointer" @click="toggleEditMode" />
        <Icon class="i-carbon:checkmark cursor-pointer" @click="applyUpdate" />
      </template>
      <template v-else>
        <Icon class="i-carbon:edit cursor-pointer" @click="toggleEditMode" />
      </template>
    </div>
  </div>
</template>

<style lang='scss' scoped>
.editable-row {
  --uno: flex items-center;

  label {
    width: 50px;
    text-align: right;
    margin-right: 1rem;
  }
}
</style>
