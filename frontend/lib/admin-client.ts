import createClient from 'openapi-fetch'
import type { paths } from './api'
import createAuthMiddleware from './admin-auth-middleware'
import type { Family, FetchedEntity } from '~/lib'

// client as a closure
export default function useClient() {
  const rawClient = createClient<paths>({ baseUrl: '/',
    credentials: 'include', // Ensures cookies are sent with the request
  })

  return {
    rawClient: rawClient,

    async bootstrap(username: string, password: string, remember_me: boolean) {
      const { data, error } = await rawClient.POST('/api/admin/session', {
        body: { password: password, username: username, remember_me: remember_me },
      })

      if (error) throw error

      // Install auth middleware to the stack. If it fails, ejects it.
      const authMiddleware = createAuthMiddleware()
      this.rawClient.use(authMiddleware)
      return data
    },

    async getFamily(): Promise<Family[]> {
      const { data, error } = await this.rawClient.GET('/api/admin/families')
      if (error) throw error
      return data
    },

    async fetchEntity(id: string): Promise<FetchedEntity> {
      const { data, error } = await this.rawClient.GET('/api/map/entities/{id}', {
        params: { path: { id } },
      })
      if (error) throw error
      return data
    },
  }
}
