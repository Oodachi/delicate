import { t, Trans } from '@lingui/macro'
import { Button, message, Popconfirm, Space, Table, Tooltip } from 'antd'
import React, { PureComponent } from 'react'
import styles from '../../taskList/components/List.less'
import PropTypes from 'prop-types'
import { Link } from 'umi'

class ExecutorGroupList extends PureComponent {
  confirm(id) {
    const { onDeleteItem } = this.props
    onDeleteItem(id)
  }

  cancel() {
    message.info('取消删除')
  }

  render() {
    const { onEditItem, onDeleteItem, onCopy, ...tableProps } = this.props

    const columns = [
      {
        title: <Trans>Sn</Trans>,
        dataIndex: 'id',
        key: 'id',
        fixed: 'left'
      },
      {
        title: '执行组名称',
        dataIndex: 'name',
        key: 'name',
        fixed: 'left',
        render: (text, row) => {
          return (
            <Tooltip title={t`Description` + ':' + row.description}>
              <a>{text}</a>
            </Tooltip>
          )
        }
      },
      {
        title: <Trans>Tag</Trans>,
        dataIndex: 'tag',
        key: 'tag'
      },
      {
        title: <Trans>Operation</Trans>,
        key: 'operation',
        render: (text, row) => {
          return (
            <Space split={'|'}>
              <a type={'link'} onClick={() => onEditItem(row)}>
                编辑
              </a>
              <Link to={{ pathname: `executorGroup/${row.id}` }}>组详情</Link>
              <Popconfirm
                title={`确定要删除执行执行组【${row.name}】吗？`}
                onConfirm={() => this.confirm(row.id)}
                onCancel={() => this.cancel()}
                okText="Yes"
                cancelText="No"
              >
                <a type={'link'} style={{ color: 'red' }}>
                  删除
                </a>
              </Popconfirm>
            </Space>
          )
        }
      }
    ]

    return (
      <Table
        {...tableProps}
        pagination={{ ...tableProps.pagination }}
        className={styles.table}
        columns={columns}
        simple
        rowKey={(record) => record.id}
      />
    )
  }
}

ExecutorGroupList.propTypes = {
  onEditItem: PropTypes.func,
  onDeleteItem: PropTypes.func,
  onCopy: PropTypes.func
}

export default ExecutorGroupList
