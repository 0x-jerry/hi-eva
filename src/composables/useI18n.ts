import { useTranslation } from 'i18next-vue'

export function useI18n() {
  return useTranslation()
}

/**
 * This function is only for IDE autocomplete
 * @param s 
 * @returns 
 */
export const t = (s: string) => s
