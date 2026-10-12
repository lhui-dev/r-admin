import request from '@/utils/request'

interface ApiResponse<T> {
  code: number
  message: string
  data: T
}

export interface PostListItem {
  id: number
  code: string
  name: string
  sort: number
  status: number
  user_count: number
  remark: string | null
  created_at: string
}

export interface PostDetailData extends PostListItem {
  updated_at: string
}

export interface PostListResponseData {
  items: PostListItem[]
  pagination: {
    page: number
    page_size: number
    total: number
  }
}

export interface PostListQuery {
  page?: number
  page_size?: number
  keyword?: string
  status?: number
}

export interface CreatePostPayload {
  code: string
  name: string
  sort?: number
  status?: number
  remark?: string
}

export interface UpdatePostPayload {
  name?: string
  sort?: number
  status?: number
  remark?: string
}

export interface PostMutationData {
  id: number
  code: string
  name: string
}

export interface PostStatusMutationData extends PostMutationData {
  status: number
}

export async function fetchSystemPosts(params: PostListQuery) {
  return request.get<ApiResponse<PostListResponseData>, ApiResponse<PostListResponseData>>(
    '/system/posts',
    { params },
  )
}

export async function fetchSystemPostDetail(postId: number) {
  return request.get<ApiResponse<PostDetailData>, ApiResponse<PostDetailData>>(
    `/system/posts/${postId}`,
  )
}

export async function createSystemPost(payload: CreatePostPayload) {
  return request.post<ApiResponse<PostMutationData>, ApiResponse<PostMutationData>>(
    '/system/posts',
    payload,
  )
}

export async function updateSystemPost(postId: number, payload: UpdatePostPayload) {
  return request.patch<ApiResponse<PostMutationData>, ApiResponse<PostMutationData>>(
    `/system/posts/${postId}`,
    payload,
  )
}

export async function updateSystemPostStatus(postId: number, status: number) {
  return request.patch<ApiResponse<PostStatusMutationData>, ApiResponse<PostStatusMutationData>>(
    `/system/posts/${postId}/status`,
    { status },
  )
}

export async function deleteSystemPost(postId: number) {
  return request.delete<ApiResponse<PostMutationData>, ApiResponse<PostMutationData>>(
    `/system/posts/${postId}`,
  )
}
