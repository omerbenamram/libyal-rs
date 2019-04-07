if HAVE_LOCAL_LIBFUSN
AM_CPPFLAGS = \
	-I$(top_srcdir)/include \
	-I$(top_srcdir)/common \
	@LIBCERROR_CPPFLAGS@ \
	@LIBCNOTIFY_CPPFLAGS@ \
	@LIBUNA_CPPFLAGS@ \
	@LIBFDATETIME_CPPFLAGS@

noinst_LTLIBRARIES = libfusn.la

libfusn_la_SOURCES = \
	fusn_record.h \
	libfusn_debug.c libfusn_debug.h \
	libfusn_definitions.h \
	libfusn_extern.h \
	libfusn_error.c libfusn_error.h \
	libfusn_libcerror.h \
	libfusn_libcnotify.h \
	libfusn_libfdatetime.h \
	libfusn_libuna.h \
	libfusn_record.c libfusn_record.h \
	libfusn_support.c libfusn_support.h \
	libfusn_types.h \
	libfusn_unused.h
endif
MAINTAINERCLEANFILES = \
	Makefile.in

distclean: clean
	/bin/rm -f Makefile

splint:
	@echo "Running splint on libfusn ..."
	-splint -preproc -redef $(DEFS) $(DEFAULT_INCLUDES) $(INCLUDES) $(AM_CPPFLAGS) $(libfusn_la_SOURCES)
