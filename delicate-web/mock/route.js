const database = [
  {
    id: '1',
    icon: 'dashboard',
    name: 'Dashboard',
    zh: {
      name: '仪表盘'
    },
    route: '/dashboard'
  },
  {
    id: '2',
    breadcrumbParentId: '1',
    name: 'Task List',
    zh: { name: '任务列表' },
    icon: 'task-list',
    route: '/taskList'
  },
  {
    id: '21',
    menuParentId: '-1',
    breadcrumbParentId: '2',
    name: 'Task Log',
    zh: { name: '任务日志' },
    route: '/taskList/:id'
  },
  {
    id: '3',
    breadcrumbParentId: '1',
    name: 'Executor Resources',
    zh: { name: '执行资源' },
    icon: 'executor-o'
  },
  {
    id: '31',
    menuParentId: '3',
    breadcrumbParentId: '3',
    name: 'Execution Node',
    zh: { name: '执行节点' },
    route: '/executor'
  },
  {
    id: '32',
    menuParentId: '3',
    breadcrumbParentId: '3',
    name: 'Execution Group',
    zh: { name: '执行组' },
    route: '/executorGroup'
  },
  {
    id: '321',
    menuParentId: '-1',
    breadcrumbParentId: '32',
    name: 'Execution Group',
    zh: { name: '组详情' },
    route: '/executorGroup/:id'
  },
  {
    id: '4',
    breadcrumbParentId: '1',
    name: 'user',
    zh: { name: '用户管理' },
    icon: 'user',
    route: '/user'
  },
  {
    id: '41',
    menuParentId: '-1',
    breadcrumbParentId: '4',
    name: 'User Detail',
    zh: {
      name: '用户详情'
    },
    route: '/user/:id'
  }
  // {
  //   id: '6',
  //   breadcrumbParentId: '1',
  //   name: 'Request',
  //   zh: {
  //     name: 'Request'
  //   },
  //   icon: 'api',
  //   route: '/request'
  // },
  // {
  //   id: '4',
  //   breadcrumbParentId: '1',
  //   name: 'UI Element',
  //   zh: {
  //     name: 'UI组件'
  //   },
  //   icon: 'camera-o'
  // },
  // {
  //   id: '45',
  //   breadcrumbParentId: '4',
  //   menuParentId: '4',
  //   name: 'Editor',
  //   zh: {
  //     name: 'Editor'
  //   },
  //   icon: 'edit',
  //   route: '/editor'
  // },
  // {
  //   id: '5',
  //   breadcrumbParentId: '1',
  //   name: 'Charts',
  //   zh: {
  //     name: 'Charts'
  //   },
  //   icon: 'code-o'
  // },
  // {
  //   id: '51',
  //   breadcrumbParentId: '5',
  //   menuParentId: '5',
  //   name: 'ECharts',
  //   zh: {
  //     name: 'ECharts'
  //   },
  //   icon: 'line-chart',
  //   route: '/chart/ECharts'
  // },
  // {
  //   id: '52',
  //   breadcrumbParentId: '5',
  //   menuParentId: '5',
  //   name: 'HighCharts',
  //   zh: {
  //     name: 'HighCharts'
  //   },
  //   icon: 'bar-chart',
  //   route: '/chart/highCharts'
  // },
  // {
  //   id: '53',
  //   breadcrumbParentId: '5',
  //   menuParentId: '5',
  //   name: 'Rechartst',
  //   zh: {
  //     name: 'Rechartst'
  //   },
  //   icon: 'area-chart',
  //   route: '/chart/Recharts'
  // }
]

export default database
