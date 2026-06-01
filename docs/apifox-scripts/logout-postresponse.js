pm.test('logout status is 200', function () {
  pm.response.to.have.status(200);
});

const json = pm.response.json();

pm.test('logout business code is 0', function () {
  pm.expect(json.code).to.eql(0);
});

pm.environment.unset('access_token');
pm.environment.unset('auth_header');
pm.environment.unset('current_user_id');
pm.environment.unset('current_username');
pm.environment.unset('current_roles');
pm.environment.unset('current_permissions');
pm.environment.unset('current_menu_titles');

console.log('auth environment cleared');
