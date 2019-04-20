function onSignIn(googleUser) {
  // The ID token needed to be passed to the backend is saved as a cookie
  var auth_response = googleUser.getAuthResponse();
  var id_token = auth_response.id_token;
  var expire_ms = auth_response.expires_in*1000; //expires_in is the # of seconds from now until the id_token expires
  var d = new Date();
  d.setTime(d.getTime() + expire_ms);
  var expires = " expires="+d.toUTCString()+";";
  document.cookies = "id_token="+id_token+";"+expires+" path=/;";
  console.log(document.cookies);
}

//For requests that need user authentication, add the following lines before the xmlhttp request is sent
//IN <head> of page w/function:
//    <script src="https://apis.google.com/js/platform.js?onload=init" async defer></script>
//    <script type="text/javascript" src="/access/signin.js"></script>
//IN function:
//    xmlhttp.setRequestHeader("id_token", getID_Token());
function getID_Token(first_run = true) {
  var name = "id_token=";
  var value = "";
  var cookies_array = document.cookie.split(';');
  for(var i = 0; i < cookies_array.length; i++) {
    var c = cookies_array[i];
    while (c.charAt(0) == ' ') {
      c = c.substring(1);
    }
    if (c.indexOf(name) == 0) {
      value =  c.substring(name.length, c.length);
      break;
    }
  }

  if (value === "" && first_run) {
    var auth = gapi.auth2.getAuthInstance();
    auth.signin().then(onSignIn);
    return getID_Token(false);
  }

  return value;
}

function signOut() {
  var auth = gapi.auth2.getAuthInstance();
  auth2.signOut();
}
