import { createMemoryHistory, createRouter } from 'vue-router'

import DashboardView from '../views/DashboardView.vue'
import AccountsView from '../views/AccountsView.vue'
import TransactionsView from '../views/TransactionsView.vue'
import BudgetView from '../views/BudgetView.vue'
import CashFlowView from '../views/CashFlowView.vue'
import ReportsView from '../views/ReportsView.vue'
import RecurringView from '../views/RecurringView.vue'
import GoalsView from '../views/GoalsView.vue'
import InvestmentsView from '../views/InvestmentsView.vue'

const routes = [
    { path: '/', redirect: '/dashboard' },
    { path: '/dashboard', component: DashboardView },
    { path: '/accounts', component: AccountsView },
    { path: '/transactions', component: TransactionsView },
    { path: '/budgets', component: BudgetView },
    { path: '/cash-flow', component: CashFlowView },
    { path: '/reports', component: ReportsView },
    { path: '/recurring', component: RecurringView },
    { path: '/goals', component: GoalsView },
    { path: '/investments', component: InvestmentsView },
]

export const router = createRouter({
    history: createMemoryHistory(),
    routes,
})

export default router