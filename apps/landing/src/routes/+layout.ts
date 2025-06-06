import { dev } from "$app/environment"

export const prerender = true // ssg
export const csr = dev // no js (enabled in dev for hmr)
