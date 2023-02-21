// dllmain.cpp : Defines the entry point for the DLL application.
#include "pch.h"

extern "C" __declspec(dllexport) int fooBar(int arg)
{
	return arg + 5;
}


