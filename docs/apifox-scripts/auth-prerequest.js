const token = pm.environment.get('access_token');

if (!token) {
  throw new Error('缺少 access_token，请先执行登录接口。');
}

const authHeader = `Bearer ${token}`;

pm.environment.set('auth_header', authHeader);
pm.request.headers.upsert({
  key: 'Authorization',
  value: authHeader,
});
