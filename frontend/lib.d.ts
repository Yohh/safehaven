// This file re-exports types from the API schema, and other convenient types
// so that they can be easily imported in other files.

import type { Coordinate } from 'ol/coordinate'
import type * as api from '~/lib/api'

export type InitConfig = api.components['schemas']['StatusResponse']

export type Entity = api.components['schemas']['Entity']
export type Cluster = api.components['schemas']['Cluster']
export type CachedEntity = api.components['schemas']['CachedEntity']

export type Family = Omit<api.components['schemas']['Family'], 'entity_form' | 'comment_form'> & {
  entity_form: api.components['schemas']['Form']
  comment_form: api.components['schemas']['Form']
}

export type PublicComment = api.components['schemas']['PublicComment']

export type Category = api.components['schemas']['Category']
export type Tag = api.components['schemas']['Tag']
export type SHComment = api.components['schemas']['Comment']

export type CartographyInitConfig =
  api.components['schemas']['CartographyInitConfig']

export type FetchedEntity = api.components['schemas']['FetchedEntity']
export type ResolvedFetchedEntity = FetchedEntity & {
  family: Family
  category: Category
  tags: Tag[]
}

export type ErrorResponse = api.components['schemas']['ErrorResponse']

export type NewCategory = api.components['schemas']['NewCategory']
export type UpdateCategory = api.components['schemas']['UpdateCategory']
export type NewOrUpdateTag = api.components['schemas']['NewOrUpdateTag']
export type NewComment = api.components['schemas']['NewComment']
export type UpdateComment = api.components['schemas']['UpdateComment']
export type NewEntity = api.components['schemas']['NewEntity']
export type UpdateEntity = api.components['schemas']['UpdateEntity']
export type ListedEntity = api.components['schemas']['ListedEntity']
export type ListedComment = api.components['schemas']['ListedComment']
export type NewOrUpdateAccessToken = api.components['schemas']['NewOrUpdateAccessToken']
export type AccessToken = api.components['schemas']['AccessToken']
export type NewOrUpdateFamily = api.components['schemas']['NewOrUpdateFamily']

export type User = components['schemas']['User']
export type NewUser = components['schemas']['NewUser']
export type ChangePassword = components['schemas']['ChangePassword']
export type SafeHavenOptions = components['schemas']['SafeHavenOptions']
export type ConfigurationOption = components['schemas']['ConfigurationOption']

export interface CanBeHighlighted {
  highlighted: boolean
}

export interface HasCoordinates {
  coordinates: Coordinate
}

export interface HasFamily {
  family: Family
}

export interface HasCategory {
  category: Category
}

export type DisplayableCachedEntity = CachedEntity &
  HasCoordinates &
  HasFamily &
  HasCategory &
  CanBeHighlighted

export type DisplayableCluster = Cluster & HasCoordinates
