const username = pm.environment.get('login_username') || 'admin';
const password = pm.environment.get('login_password') || 'Admin@123456';

pm.variables.set('request_login_username', username);
pm.variables.set('request_login_password', password);
pm.variables.set(
  'request_login_body',
  JSON.stringify({
    username,
    password,
  })
);
