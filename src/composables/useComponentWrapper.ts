import {
  type Component,
  defineComponent,
  h,
  type MaybeRefOrGetter,
  mergeProps,
  shallowRef,
  toValue,
} from 'vue'

type GetComponentProps<T> = GetComponentExpose<T> extends {
  $props: infer Props
}
  ? Props
  : never

type GetComponentExpose<T> = T extends { new (): unknown }
  ? InstanceType<T>
  : never

export interface UseComponentWrapperOptions<T> {
  props: MaybeRefOrGetter<Partial<GetComponentProps<T>>>
}

export function useComponentWrapper<T extends Component>(
  Component: T,
  opt?: UseComponentWrapperOptions<T>,
) {
  const name = (Component as any).name || 'Unknown'

  const instanceRef = shallowRef<GetComponentExpose<T>>()

  const WrapperComponent = defineComponent({
    name: `${name}Wrapper`,
    render() {
      const { $attrs, $slots } = this

      const props = mergeProps($attrs, toValue(opt?.props || {}))

      return h(Component, props, $slots)
    },
  })

  return {
    Component: WrapperComponent as T,
    get instance() {
      return instanceRef.value
    },
  }
}
