#include "../include/types.h"
#include "../include/user.h"
#include "../include/stat.h"

int
main(int argc, char *argv[])
{
  int pid_input = (int)argv[1][0];
  if(pid_input == -13){
    ps(0);
  }else{
    ps(pid_input-'0');
  }
  exit();
}