var isValid = function(s) {
    if(s == null||s.length <1){
        return true;
    }
    var stack = [];
    for(var i=0;i<s.length;i++){
        if(s[i] == ""){
            continue;
        }
        if(stack.leng<1){
            stack.push(s[i]);
        }else if(stack[stack.length-1]=="("&&s[i]==")" ||stack[stack.length-1]=="{"&&s[i]=="}" ||stack[stack.length-1]=="["&&s[i]=="]"){
            stack.pop();
        }else{
            stack.push(s[i]);
        }
    }
    return stack.length<1;
};
