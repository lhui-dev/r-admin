pm.test('login status is 200', function () {
  pm.response.to.have.status(200);
});

const json = pm.response.json();

pm.test('login business code is 0', function () {
  pm.expect(json.code).to.eql(0);
});

pm.test('login payload contains token and user', function () {
  pm.expect(json.data).to.have.property('access_token');
  pm.expect(json.data).to.have.property('user');
  pm.expect(json.data.user).to.have.property('username');
});

pm.environment.set('access_token', json.data.access_token);
pm.environment.set('auth_header', `Bearer ${json.data.access_token}`);
pm.environment.set('current_user_id', String(json.data.user.id));
pm.environment.set('current_username', json.data.user.username);
pm.environment.set('current_roles', JSON.stringify([]));
pm.environment.set('current_permissions', JSON.stringify([]));

console.log('login user:', json.data.user.username);
console.log('token saved to environment');
