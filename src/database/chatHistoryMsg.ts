import { bindAllMethodsToSelf } from '../utils'
import { type BaseModel, BaseModelManager, COMMON_COLUMN } from './database'
import type { GetPureModelType } from './types'

export interface IChatHistoryMsgModel extends BaseModel {
  role: string
  content: string

  chatHistoryId: number
}

export type IChatHistoryMsgItem = GetPureModelType<IChatHistoryMsgModel>

class ChatHistoryMsgTable extends BaseModelManager<IChatHistoryMsgModel> {
  TABLE_NAME = 'chat_history_msg'
  COLUMN_NAMES = ['role', 'content', 'chatHistoryId']

  async getMsgs(chatHistoryId: number) {
    const sql = `select * from ${this.TABLE_NAME} where chatHistoryId = $1 order by ${COMMON_COLUMN.createdDate} desc`

    const result = await this.db.select<IChatHistoryMsgModel[]>(sql, [
      chatHistoryId,
    ])

    return result
  }

  async getByContent(content: string) {
    const sql = `select * from ${this.TABLE_NAME} where content = $1`

    const resp = await this.db.select<IChatHistoryMsgModel[]>(sql, [content])

    return resp.at(0)
  }
}

export const chatHistoryMsgTable = new ChatHistoryMsgTable()
bindAllMethodsToSelf(chatHistoryMsgTable)
