import type { App } from 'vue'
import {
    createRouter,
    createWebHashHistory,
    RouteLocationNormalized,
    RouteRecordRaw
} from 'vue-router'
import { ref } from 'vue'
import Home from '@/views/index/index.vue'
import AppConfig from '@/config'
import { useUserStore } from '@/store/modules/user'
import { menuService } from '@/api/menuApi'
import { useMenuStore } from '@/store/modules/menu'
import { useSettingStore } from '@/store/modules/setting'
import NProgress from 'nprogress'
import 'nprogress/nprogress.css'
import { useTheme } from '@/composables/useTheme'
import { setWorkTab } from '@/utils/worktab'
import { MenuListType } from '@/types/menu'
import { processRoute } from '@/utils/menu.ts'
import { ApiStatus } from '@/utils/http/status.ts'
import { ElMessage } from 'element-plus'
import { RoutesAlias } from './modules/routesAlias'
import { registerAsyncRoutes } from './modules/dynamicRoutes'

/** 顶部进度条配置 */
NProgress.configure({
    easing: 'ease',
    speed: 600,
    showSpinner: false,
    trickleSpeed: 200,
    parent: 'body'
})

/** 扩展的路由配置类型 */
export type AppRouteRecordRaw = RouteRecordRaw & {
    hidden?: boolean
}

/** 首页路径常量 */
export const HOME_PAGE = '/dashboard/console'

/** 静态路由配置 */
const staticRoutes: AppRouteRecordRaw[] = [
    { path: '/', redirect: HOME_PAGE },
    {
        path: '/dashboard',
        component: Home,
        name: 'Dashboard',
        meta: { title: '仪表盘' },
        children: [
            {
                path: RoutesAlias.Dashboard,
                name: 'Console',
                component: () => import('@/views/dashboard/console/index.vue'),
                meta: { title: '工作台', keepAlive: false }
            },
            {
                path: RoutesAlias.Analysis,
                name: 'Analysis',
                component: () => import('@/views/dashboard/analysis/index.vue'),
                meta: { title: '分析页', keepAlive: false }
            },
            {
                path: RoutesAlias.Ecommerce,
                name: 'Ecommerce',
                component: () => import('@/views/dashboard/ecommerce/index.vue'),
                meta: { title: '电子商务', keepAlive: false }
            }
        ]
    },
    {
        path: RoutesAlias.Login,
        name: 'Login',
        component: () => import('@/views/login/index.vue'),
        meta: { title: '登录', isHideTab: true, setTheme: true }
    },
    {
        path: RoutesAlias.Register,
        name: 'Register',
        component: () => import('@/views/register/index.vue'),
        meta: { title: '注册', isHideTab: true, noLogin: true, setTheme: true }
    },
    {
        path: RoutesAlias.ForgetPassword,
        name: 'ForgetPassword',
        component: () => import('@/views/forget-password/index.vue'),
        meta: {
            title: '忘记密码',
            isHideTab: true,
            noLogin: true,
            setTheme: true
        }
    },
    {
        path: '/exception',
        component: Home,
        name: 'Exception',
        meta: { title: '异常页面' },
        children: [
            {
                path: RoutesAlias.Exception403,
                name: 'Exception403',
                component: () => import('@/views/exception/403.vue'),
                meta: { title: '403' }
            },
            {
                path: '/:catchAll(.*)',
                name: 'Exception404',
                component: () => import('@/views/exception/404.vue'),
                meta: { title: '404' }
            },
            {
                path: RoutesAlias.Exception500,
                name: 'Exception500',
                component: () => import('@/views/exception/500.vue'),
                meta: { title: '500' }
            }
        ]
    },
    {
        path: '/outside',
        component: Home,
        name: 'Outside',
        meta: { title: '内嵌页面' },
        children: [
            {
                path: '/outside/iframe/:path',
                name: 'Iframe',
                component: () => import('@/views/outside/Iframe.vue'),
                meta: { title: 'iframe' }
            }
        ]
    }
]

/** 创建路由实例 */
export const router = createRouter({
    history: createWebHashHistory(),
    routes: staticRoutes,
    scrollBehavior: () => ({ left: 0, top: 0 })
})

// 标记是否已经注册动态路由
const isRouteRegistered = ref(false)

/**
 * 路由全局前置守卫
 * 处理进度条、获取菜单列表、动态路由注册、404 检查、工作标签页及页面标题设置
 */
// eslint-disable-next-line consistent-return
router.beforeEach(async (to, _, next) => {
    const settingStore = useSettingStore()
    if (settingStore.showNprogress) NProgress.start()

    // 设置登录注册页面主题
    setSystemTheme(to)

    // 检查登录状态，如果未登录则跳转到登录页
    const userStore = useUserStore()

    if (!userStore.accessToken) {
        userStore.logOut().then(() => {
            return next('/login')
        })
    }

    if (!userStore.isLogin && to.path !== '/login' && !to.meta.noLogin) {
        userStore.logOut().then(() => {
            return next('/login')
        })
    }

    // 如果用户已登录且动态路由未注册，则注册动态路由
    if (!isRouteRegistered.value && userStore.isLogin) {
        try {
            await getMenuData()
            // 标记路由已注册
            isRouteRegistered.value = true
            if (to.name === 'Exception404') {
                return next({ path: to.path, query: to.query, replace: true })
            }
            return next({ ...to, replace: true })
        } catch (error) {
            console.error('Failed to register routes:', error)
            return next('/exception/500')
        }
    }

    // 检查路由是否存在，若不存在则跳转至404页面
    if (to.matched.length === 0) {
        return next('/exception/404')
    }

    // 设置工作标签页和页面标题
    setWorkTab(to)
    setPageTitle(to)

    next()
})

/**
 * 根据接口返回的菜单列表注册动态路由
 * @throws 若菜单列表为空或获取失败则抛出错误
 */
async function getMenuData(): Promise<void> {
    try {
        // 获取菜单列表
        const menuListResponse = await menuService.getMenuList()
        console.log('menuListResponse:', menuListResponse)
        if (menuListResponse.code !== ApiStatus.success || menuListResponse.data.length === 0) {
            await useUserStore().logOut()
        }

        const menuList: MenuListType[] = menuListResponse.data.map((route) => processRoute(route))

        // 如果菜单列表为空，执行登出操作并跳转到登录页
        if (!Array.isArray(menuList) || menuList.length === 0) {
            await useUserStore().logOut()
            ElMessage.error('获取菜单列表失败，请重新登录！')
        }

        // 设置菜单列表
        useMenuStore().setMenuList(menuList as [])
        // 注册异步路由
        registerAsyncRoutes(router, menuList)
    } catch (error) {
        console.error('获取菜单列表失败:', error)
        throw error
    }
}

/* ============================
   路由守卫辅助函数
============================ */

/**
 * 根据路由元信息设置系统主题
 * @param to 当前路由对象
 */
const setSystemTheme = (to: RouteLocationNormalized): void => {
    if (to.meta.setTheme) {
        useTheme().switchThemeStyles(useSettingStore().systemThemeType)
    }
}

/**
 * 设置页面标题，根据路由元信息和系统信息拼接标题
 * @param to 当前路由对象
 */
export const setPageTitle = (to: RouteLocationNormalized): void => {
    const { title } = to.meta
    if (title) {
        setTimeout(() => {
            document.title = `${String(title)} - ${AppConfig.systemInfo.name}`
        }, 150)
    }
}

/** 路由全局后置守卫 */
router.afterEach(() => {
    if (useSettingStore().showNprogress) NProgress.done()
})

/**
 * 初始化路由，将 Vue Router 实例挂载到 Vue 应用中
 * @param app Vue 应用实例
 */
export function initRouter(app: App<Element>): void {
    app.use(router)
}
