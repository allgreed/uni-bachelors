#include <X11/Xlib.h>
#include <X11/X.h>
#include <stdio.h>
#include <stdlib.h>
#include <sys/types.h>
#include <sys/wait.h>
#include <time.h>
#include <unistd.h>


int oknuj(int id, char* host);


int main(int argc, char *argv[])
{
    clock_t start = clock();

    for(int i = 1; i < argc; ++i)
    {
        if (fork() == 0)
        {
            oknuj(i, argv[i]);
            exit(0);
        }
    }

    int child_exit_stats = 0;
	while (wait(&child_exit_stats) > 0)
    {
		clock_t end = clock();
        clock_t tick_diff = end - start;

		int n = child_exit_stats >> 8;
		printf("Window %d at %s was closed after %ld ticks\n", n, argv[n], tick_diff);
	}
}


int oknuj(int id, char* host){

	Display * display = XOpenDisplay(host);
	int screen = DefaultScreen(display);
	GC gc = DefaultGC(display, screen);
	Visual * visual = DefaultVisual(display,screen);
	int depth = DefaultDepth(display,screen);
	Colormap colormap = DefaultColormap(display,screen);

    XColor rush, _;

    XAllocNamedColor(display, colormap, "pink", &rush, &_);

	XSetWindowAttributes windowattributes = { .background_pixel = rush.pixel, .override_redirect = False };

	Window window = XCreateWindow(display, XRootWindow(display, screen), 100, 100, 500, 700, 10, depth, InputOutput, visual, CWBackPixel|CWOverrideRedirect, &windowattributes);
	XMapWindow(display, window);
	XSelectInput(display, window, ExposureMask|KeyPressMask);


    XEvent event;
	while (1)
    {
        XNextEvent(display, &event);
        if (event.type == KeyPress)
        {
            XCloseDisplay(display);
            exit(id);
		}
	}
}
