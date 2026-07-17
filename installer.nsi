Unicode true

!define PRODUCT_NAME "记账APP"
!define PRODUCT_VERSION "1.0.0"
!define PRODUCT_PUBLISHER "记账APP"
!define PRODUCT_EXE "expense-tracker.exe"

SetCompressor lzma

!include "MUI2.nsh"

!define MUI_ABORTWARNING

!insertmacro MUI_PAGE_WELCOME
!insertmacro MUI_PAGE_DIRECTORY
!insertmacro MUI_PAGE_INSTFILES
!insertmacro MUI_PAGE_FINISH

!insertmacro MUI_UNPAGE_CONFIRM
!insertmacro MUI_UNPAGE_INSTFILES

!insertmacro MUI_LANGUAGE "SimpChinese"

Name "${PRODUCT_NAME} ${PRODUCT_VERSION}"
OutFile "C:\Users\HUAWEI\AppData\Local\app-pkg\记账APP_Setup.exe"
InstallDir "$PROGRAMFILES64/${PRODUCT_NAME}"
RequestExecutionLevel admin

Section "安装"
  SetOutPath "$INSTDIR"
  File "C:\Users\HUAWEI\AppData\Local\app-pkg\${PRODUCT_EXE}"
  CreateDirectory "$SMPROGRAMS/${PRODUCT_NAME}"
  CreateShortCut "$SMPROGRAMS/${PRODUCT_NAME}/${PRODUCT_NAME}.lnk" "$INSTDIR/${PRODUCT_EXE}"
  CreateShortCut "$DESKTOP/${PRODUCT_NAME}.lnk" "$INSTDIR/${PRODUCT_EXE}"
  WriteUninstaller "$INSTDIR/uninst.exe"
  WriteRegStr HKLM "Software\Microsoft\Windows\CurrentVersion\Uninstall\${PRODUCT_NAME}" "DisplayName" "${PRODUCT_NAME}"
  WriteRegStr HKLM "Software\Microsoft\Windows\CurrentVersion\Uninstall\${PRODUCT_NAME}" "UninstallString" "$INSTDIR/uninst.exe"
  WriteRegStr HKLM "Software\Microsoft\Windows\CurrentVersion\Uninstall\${PRODUCT_NAME}" "DisplayVersion" "${PRODUCT_VERSION}"
  WriteRegStr HKLM "Software\Microsoft\Windows\CurrentVersion\Uninstall\${PRODUCT_NAME}" "Publisher" "${PRODUCT_PUBLISHER}"
SectionEnd

Section "Uninstall"
  Delete "$INSTDIR/${PRODUCT_EXE}"
  Delete "$INSTDIR/uninst.exe"
  RMDir "$INSTDIR"
  Delete "$SMPROGRAMS/${PRODUCT_NAME}/${PRODUCT_NAME}.lnk"
  RMDir "$SMPROGRAMS/${PRODUCT_NAME}"
  Delete "$DESKTOP/${PRODUCT_NAME}.lnk"
  DeleteRegKey HKLM "Software\Microsoft\Windows\CurrentVersion\Uninstall\${PRODUCT_NAME}"
SectionEnd
