import { type BaseModel, BaseModelManager } from './database'
import type { GetPureModelType } from './types'

export interface IPromptConfigModel extends BaseModel {
  name?: string
  icon?: string
  prompt?: string
  isBuiltin?: number
  endpointId?: number
}

export type IPromptConfigItem = GetPureModelType<IPromptConfigModel>

class PromptConfigTable extends BaseModelManager<IPromptConfigModel> {
  TABLE_NAME = 'prompt_config'
  COLUMN_NAMES = ['name', 'icon', 'prompt', 'isBuiltin', 'endpointId']
}

export const promptConfigTable = new PromptConfigTable()
