import { type BaseModel, BaseModelManager } from './database'

export interface IPromptConfigModel extends BaseModel {
  name: string
  icon: string
  prompt: string
  isBuiltin: boolean
}

class PromptConfigTable extends BaseModelManager<IPromptConfigModel> {
  TABLE_NAME = 'prompt_config'
  COLUMN_NAMES = ['name', 'icon', 'prompt', 'isBuiltin']
}

export const promptConfigTable = new PromptConfigTable()
