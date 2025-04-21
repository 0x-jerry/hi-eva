import dayjs from 'dayjs'
import Database from '@tauri-apps/plugin-sql'
import { snakeCase } from 'lodash-es'

export const db = Database.get('sqlite:data.db')

window.addEventListener('beforeunload', async () => {
  await db.close()
})

export interface BaseModel {
  id: number
  createdDate: number
  updatedDate: number
}

interface PaginationParam {
  size?: number
  current: number
}

type CreatedModel<T extends BaseModel> = Omit<
  T,
  'id' | 'createdDate' | 'updatedDate'
>

type UpdatedModel<T extends BaseModel> = Omit<T, 'createdDate' | 'updatedDate'>

const COMMON_COLUMN = {
  id: 'id',
  createdDate: 'createdDate',
  updatedDate: 'updatedDate',
} as const

const commonColumn = [COMMON_COLUMN.createdDate, COMMON_COLUMN.updatedDate]

export abstract class BaseModelManager<T extends BaseModel> {
  abstract readonly TABLE_NAME: string

  /**
   * Only custom columns, no need to include id/createdDate/updatedDate.
   *
   * Must use camel case.
   */
  abstract readonly COLUMN_NAMES: string[]

  readonly db = db

  async page(opt: PaginationParam) {
    const { size = 10, current } = opt

    const resp = await db.select<T[]>(
      `select * from ${this.TABLE_NAME} limit ${size} offset ${size * current}`,
    )

    return resp
  }

  async getById(id: number) {
    const resp = await db.select<T[]>(
      `select * from ${this.TABLE_NAME} where id = $1`,
      [id],
    )

    return resp.at(0)
  }

  async deleteById(id: number) {
    const resp = await db.execute(
      `delete from ${this.TABLE_NAME} where id = $1`,
      [id],
    )

    return resp
  }

  async createOne(data: CreatedModel<T>) {
    const columns = [...commonColumn, ...this.COLUMN_NAMES]

    const columnNames = columns.map((name) => snakeCase(name)).join(', ')
    const placeholders = columns.map((_, idx) => `${idx + 1}`).join(', ')

    const values = [
      ...commonColumn.map((_) => dayjs().unix()),
      ...this.COLUMN_NAMES.map((key) => Reflect.get(data, key)),
    ]

    const resp = await db.execute(
      `insert into ${this.TABLE_NAME} (${columnNames}) values (${placeholders})`,
      values,
    )

    // biome-ignore lint/style/noNonNullAssertion: lastInsertId always exists.
    return await this.getById(resp.lastInsertId!)
  }

  async updateOne(data: UpdatedModel<T>) {
    const columns = [COMMON_COLUMN.updatedDate, ...this.COLUMN_NAMES]

    const placeholders = columns
      .map((name, idx) => `${snakeCase(name)} = ${idx + 1}`)
      .join(', ')

    const values = [
      dayjs().unix(),
      ...this.COLUMN_NAMES.map((key) => Reflect.get(data, key)),
      data.id,
    ]

    const resp = await db.execute(
      `update ${this.TABLE_NAME} set ${placeholders} where id = ${columns.length + 1}`,
      values,
    )

    return resp
  }
}
