import type { BaseModel } from './database'

export type GetPureModelType<T> = (T extends BaseModel
  ? Omit<T, keyof BaseModel>
  : T) &
  Partial<BaseModel>
