// Copyright Claudio Mattera 2021.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! Palette control

use crate::wasm4::PALETTE;

/// Screen palette
///
/// WASM-4 uses a 4-colour palette, i.e. at a given time there can only be at
/// most 4 different colours on the screen.
///
/// This enum defines several 4-colour palettes.
pub enum Palette {
    // Generate colour placeholders at https://placeholderimage.dev/

    /// Default palette of WASM-4
    ///
    /// 1. `#def7cd` <img src="data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAADIAAAAPCAYAAACm25zCAAACMUlEQVRIiW2WQRbEIAhDe/+z6RX0FrB0FhX4obOY12mLCEmIfbaPs22e5eMsG2f7PNvHe3/fbZvybvs8y+dZNt//Ns6yex//fdzYuy6fRUzEve/enOPmHLknc61P3Mwan+UMjGYGgkYV1RrdjG/FLTQfMbXmNoNCdq6Z2LOalzW5Vhq5BXgr4lM8UOXVCvFsxtFErHVs3gu1yFl5hH0D0NkUAbqMRKL3Wogp+hUXyId0spjOTBYHkELCVsBkbOaY2pxIqwBYt4mXERtS4Eayon1kwmVgMJ8FY1VEzcPUZ7nHzL36bEoeU0YUzJLds0CfyABDV00BjdY8WRO5RFEhTYM0vKSSzLkOdFdHAtuaeZRyHXwZOlP0uaE0m2xqYZsSExkR6T8AQk5f96s8zwa6SncVs9FkPueAi7wKqZUbt1/IDVKrWeruRvOpejLvXf/US87CBHUTSE0gSompDfcC9cwAE5T1H1mq8cBYGJPDTpeg/1uhSU3SBmsOICPxe6COWRAZp8vRCPSwXACZts+anlzoihA7pv8r5W1Yxfu/Q1tnDpkYxay34rt0yUIzJbVfDGgVrp8s8ulhmix1zINQpIdC6WByFpVlc+60LhrCnZHuNOLZOHB4xnSrXSiYaFezzUjkMwP58v6fXNkEDsob84hFgpmiE6eoKUsbMZyDfpBtMkv2uyt2+bV5SykiLvKk/fZNk1aZnf8WK1/Oxk2nAKDfdHpm1MBrw3raEzw4pM/zA4GCoLscbOvAAAAAAElFTkSuQmCC">
    /// 2. `#86bf6b` <img src="data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAADIAAAAPCAYAAACm25zCAAACMUlEQVRIiW2WUbbEIAhDXa/uQ3ZVltb5aCU3dD563hurSEIIHXGte1/rjmveO9cd17p3Pms7Z/0N7NnXfJ/n/7jWHe/eeH9vrD1x5/PkczbS39e+PGd1pnJ5z8TFeM/+EUi4EnqfSIDg+qWkLamWyK73LwHpBBTAE5PJM2YRKILOmUPMqBfn8lyN4VVJ6OASKylABhTAvg9BTxAJ1vNUG2qo+wTqKGIUAEgsENwSLMZQxcNyChwlsFPSqcogif2n2pGtagmSU3dRHSPaYjGQQFysv/sqIWiePUQgkIU03SQGCQbWRGjrD6jm3DuULB9VwiWkcguUV0kNiv64nMkClNxHsJKmenFh30QuT5zBEktOyw5ETis/pcKkpWmsWa8BMHur7bV7UZVuKFXtXPdQMFkgS1lSYsCSEV3rsA2jsP6g48Bum61TWg6QspQ6Tvxh9tc13LTdZ4U3puYApRO4NFCRjXjVvNb4mD9n7WMeIn7Ui+zM0GGmsS37m98EeoWa+9VZmykcgj5US4boX/brATgCqOlGPrllvz53GnvV7D4QuxxN/+ZS368HjgLlgT2nR6R7OFZrKE7RaAA1ZZuU/liwmQPBMwc4EQfvBpkVH/Ia+8NSG0BN4zXle5lpDLBcqxwBWl8sA/Ydrv4VIPDKbez0QHQm9gsrJnCswr/K8Mw0Yqy6Zq997vgMIxj23PCv3bYZQ82+b3oSbMij6SKoeX/JlzNLku530P34CWWfUrnuH0BH23R1V19jAAAAAElFTkSuQmCC">
    /// 3. `#306950` <img src="data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAADIAAAAPCAYAAACm25zCAAACPklEQVRIiW2W0bXEIAhELQk70S6w/wLcjwS4Q/Yj571VI8wwDBl21rWzrvn7nH1jbb6/57s2fd/p+5qvO8961v154vc8+05f+eR9jjhxLs/j/vfM8379NsS1zOON7euOTOJdmOcN7vsJGBc4A6xMmCRMgo8k3sBJiCv4TO500Ih/iig7JDHuXndUUjvBkLVAHBcYLwDITFJYj0QLVIJ1kCVAUCUvIiJ2VYpr+w5LZuuQgVVLJrYATkkeyCAAO+W4hAwCtCRoJ8MkhTIKtYR6LGM8743Sax1IaTStT8ogz9X/UY3sDSdQPSf95iUbS7Z3A11gDSRFrBHlrGeBvQW5bLAFljNA66VTQFmxqjKa+Oh+VQjmAemmrIIY33dMsGAtSekJAqTDpXOE3EAI3i0D2MJwKaH6R3tKKyS9lNVcd5gkq6yXfSorbGLuf6ThlEgDC+vM3pN70WutL1X2ab9oIlpkdyEGPV0+TEi9Xklp0gMYgmPvfRo+k98Sb7DEBnZkyLmWMkH5/pxXN9IhWJLjHRx2Wk1ZT0elQmptiFdLU3F+tOHXNJu9BJnKZD5KjBiErP3pV/QNK8beewYiGrPY7rbHqduSCla4B3ciwNkSl2SlgpBkMwlRCfpm1DTHEOvBWFp6PHSqCWKw4h1KcSaQci4hpFVXZ1vkXDkN+aCDR1eFvkZQSdY+Xc8k2dpPe20DtD5N0GOY3own8oS8hnzl9r9/5kAx2JgCk58ZFInQhr2Sn60K8ilympxS0txb9wcZapd/uyyJHAAAAABJRU5ErkJggg==">
    /// 4. `#071821` <img src="data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAADIAAAAPCAYAAACm25zCAAACMElEQVRIiXWWSbblIAxDGYUtmBnsf5GpQbB15fdrwEkHbmRZznhiv9867xPnnWu/M873ru6/988674xc+1vr2/es/c517sq997rS3nf2WdffOtfvLr/1/dp7AjGE7OdKW+O5TusAAwoEmY7rOb/BQSZ895azShhAXBuZbCY1YV+Jue8nbaaf2O+Ytxo/qIUHmYYYaDnMxGirEjnYfyr5XF6BLVYsgeLJwEYI9FFZo+SFRjcG6lX1ClEgxUpUgrR5BACAmwC07MZxGgII7h8/BxNx0EqHEGg0CoZQzPeJmlUS/ciKcP9s+6riqPDk9zg3kUU06NypZb0QRPM3KNn9X8UUjNE1FI8SUS+UGJmIoEfId5bUFch7hA7IW08MVIXiPXgnZHcDcv+RJCuXqrjfkTd1jdPKKJmTtHpj6gxoYMF1u1RB7xsiL/uScR8HignNTieuGgpczyUK1fw+O57VEiDXC01QCFSZBgCSIAhNjIbRKJRE9U01HrVbg4jJk0augG24NdnWTKE/UAhybvMGlB7WB2h6NpRP2F96iOcCQUmRklBDBljCgcQ43TkkbdZIWYc5Acedz9sDgPRSjiWrf8jkaueNBQAM1e6/Quql5nddavXJORui/LWw6WoO+lzpcn2sMs55Bk/1xJVSXe8VjycSQLIPOvx+kJ/qKUhrUejYPs2r5gcD11FvA7NPe/TvMFllGcMP8FfFGq3J86/suuIpAaggnq1P0T8WuAH33f8DbBSlY6gxwGsAAAAASUVORK5CYII=">
    Default,

    /// [Gold GB](https://lospec.com/palette-list/gold-gb) palette
    ///
    /// 1. `#cfab51` <img src="data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAADIAAAAPCAYAAACm25zCAAACMUlEQVRIiX2Wy7UkIQxDCRHShFwgpJpFY+vKvDOLPvXpAtuSLNP2HN+e/TtrfHv17+Rz//Yc3173+X5zVs/fXlh3r3uN/Db2+f03dJ1lnxlr+o018t2+73bmF3E78h5fi5so6Jf8XRCJ3Hc7AkTCeY0iGBTALBQ2AULda44ES/uNF6x5wZgCr22gFz8GCFZUmBZboGBvqVgGIqKZRLAPFZxEXczpO30bz1FwE9pgYQLJfI+E45fIdtwHUsOCnQRj6H5JPmTu5PfYZ4mFWJuSnONXiOuVQaVfpxbsTElB6zsKCblcJFM2RNb7UxL2PjM2Cdq8PWKBFxGCdoEgNXvAiCQiJK23ZkF+itEHDDKWa53xACgZMdd4nGcUGt211LRsRHctyRDORpmaMXhzqy9GFvrmOr7mDtQf2syRVkUJ7lHQle26nSbSxcmY1AF7ZO7pFfzXRDd1SQfCBmbFaEhYbqKaVg5giGYmovmy/wDF2XwLjx5t1nCGwngKqJLyQaoZwqH1uAytflEyA2B0rHGrlYu6rNsmcqb/0uzsHXsG+nAoFV76zyz5ZfYgOcbLeQZr5t5NiVTLBIU5mNADlEFtajSm+sYdUAWj+dFbulaFUGaK1zQL/Bgi63Rd25kH/eXJqddei8UcYDzIy6w45hYdawqckFczOimt6tfU+lKx/r44j7kh2C3DtB4WTQFgpp79OAMb3cLPMfRyP668BlAaGqzq/FSamTKCvP83wZ+hjGH5D4SPJ++vQcmHAAAAAElFTkSuQmCC">
    /// 2. `#9d654c` <img src="data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAADIAAAAPCAYAAACm25zCAAACLUlEQVRIiXWWUZbFIAhDXaDuQnala+18KHCDbz562lOVQhJC27bxLRvfnuPbs5+75fOa49vWzzX7twzvZv+W77Nc8/Vl/cSyXNfYfj6/tabncmPMEyNizptv5HPetQjIZBnwBvOE47AnOXnPOOvG5Qfj/I3NPZEczzpwN4bvy29kzMZkVrACNi5DiwUHA17kwH5HjUhi3XDFuUQ2Cg42Mk4A6M+XnWX9a4sFCPVd6SyIh6xYjOUHyOwqLMrzBNpkZ2JdGFdAshCibgM0j7zPkkQEyo9mj9zCqlQgnxVxujJJ7U9lLYvNvJhrC/3NIdQSCWq9FpjsJTMpOcrE0WMzD+xJNRCUKAbM01D8aq7BjSI2GrM2HPXMdXGdKBSsuAxK0ezBcE2RdHG92QvTZ09boHobgk+8AzoVqWQAffS4TsZbpmvCjOk7OWPJSjKYeaRroTp/5vtkaSQSxeeZsJgGncoGAOvBNFlL54QyfvUVcmz0f2WAjcsG/l/r0fgYdOlyhVXYqMjlBztP/wFwX4+BWKVERCmX6BU0oaP19FpJ4JkhfAeL5l/AKnlxDFBm7ZncovchlL6OpDNE3EukqG4nKpCCqgGkErTxadfe7Ehul+Qe1ygzh0YhQw++zzlCzXNqb7JQeyxiYE4ZAT+5NJms8HU6D3votcnxA30ykOdoodq07B0OvNeIAlj5e8C/lvwHocGIyC57aiNyfuh/Uxl8ZJUIywjAL1FxqSUAnvN/pnW8goqkUwIAAAAASUVORK5CYII=">
    /// 3. `#4d222c` <img src="data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAADIAAAAPCAYAAACm25zCAAACJ0lEQVRIiW2WQRbEIAhDPYtu9SZ6/+M4iynkh3bV9qkYkgBtu697xrynr7vHvHuse/p6vtc9ff7Xx7ynz7v7ujxzxtKZZ9/uC3uf2Hzm/pXve/zP74gxVp63tR7P5+4Hdzu4IAEYyKlnAmFiuAjBE2CP5LC/zyRgDxATpOQa4j9PYQu8//3/RJCZXzChDJWazyXLkldS2mNJVJbtHUSA/YN1fQdm3dmS3UE1ALTIfABuj8ryc5mBBDkRN9VTnA3CpAYIDEKSVLduk+QrGTSFwhpRP5H0qOysTOrwvagihgXo9K/z5XvoW2rLxo2yGoiP2lAiVKL4dwgYa4AKHlNJtfAiEEmaO6zY//FaXmbSuUVoPdaUugvsRJv2CpAd7q2CMMglhgG2OyBdimRhT7AK9mA/f18G1hODnYbHzsbA+iixkhAonODREMIVzXxfC41K9co+GaMNqBrOWWvF2VHZlpVonW9VNOva2xZTF2CAsbhqqz6d+8sswAAV+GkJVUWs25nyslIm9+xvZ7yletUB5olN9jLAlLDbzaxahpnF/rA0FalqseE0m6JosexAZNzZh3rVSuZrtPe0CKzaK4lRS8sIsQ5pscNaNuzqECqMWytemTiDHlxo/2BlQtu0/qpDxN9I3khLa8G3BK0BpoJ/e5fDi/MIHaj43mN/J8gWrnURyGQi6UavWW/Pjej5+LWwOcGESp3VbqaG8gYdAPk3zmGt3xxhi8R+CmGyxyO4aogAAAAASUVORK5CYII=">
    /// 4. `#210b1b` <img src="data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAADIAAAAPCAYAAACm25zCAAACMUlEQVRIiXWWQZYEIQhDXesNdIP3P6SzKIEf2lnMq+kqlUBCsNmYx8Y81tfZ/XtaX2ePdWzMs8f3fvs6/P7WrNgT68fEu3Wsz/t3Y2Dv7t/378xVnjPO3yV2vLtxmzmofg8VEBeAB/SDIuEL6u4NANi7ueYWx8+JGCP3eGJM3jGYx2HiF1vbUTGvjgeZWdGRjOzCHgPzcO41r34AzqSFtUEW/EwUccxnAXafpwW1Drhr4AQ/FXAwxLUAMCi/lKVIONhZYKlI6yF5kd1d07wSSRs0+QOIWl3CiOq7/I/KGX5rATVuYPIEoxdL/OE9UhtvzB8QydAKug1VCXmigkyGrMp5bPQAiPP5hLwtsCWLTfQfH1PnoXckaY/ACYb9BG0XuVhhItb3Kawkwyl1MZqbWKtutP/J2JDYHrpOnC+kMGUvGzXBp1ykQPz2sPd01EyoBf3SD1OYYSNTbmKBRdMprYeVcpb0BCe9JRKnG8KiwXh7+juDj5IgrdCDlUNFWj8DD7EwdGk4r36LRKNo3BeMsEcSYFieHMYqw8kiCPtMCyExEPd3XsDpUNQ0F9rx976F/sSpkFB1GBxilALBYM78XlVUNmmnYJd9gzmU9l1tep4mDlFkRTmx6ryaCBCpfrlyyKyCAXA2iRxZ1JRQdUJPsrGZeG+q05pDSX09K05gnBcGlhJ4dUnaN12uTH8oJFQgzf4Ak/esYoUOsPSTXOggJzIaCcQesF/dLgZpSVT688P0B15CtuSJhb+PAAAAAElFTkSuQmCC">
    Gold,

    /// [Ice Cream GB](https://lospec.com/palette-list/ice-cream-gb) palette
    ///
    /// 1. `#fff6d3` <img src="data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAADIAAAAPCAYAAACm25zCAAACMUlEQVRIiX2WQRbcIAxDuf/x4C6ww10EW1+etou8ZDKAbVmSM+KsiD3jnhWxV9z97uc9nxn3zIi9Is6q53u0pu65Zs+4O9e95zMV5535XYyXcaZ+v/WR+byzv3gZZ8YIBskFubkWI1gGQWL3ZMAv0Bf4JbVfwnXGVKIFwHu3Udz7P3Jf5gEwA0CMS8RzQyb23qkoD1rIVpCO/CrEMvFA0ToLMSr2VAcAhEB/oGVHqvV7ihakDzeQWiiKIJCCohESy2LPss6Jih6vzm80JohxVgy10Td27heloIXq2G77D94VkuA+YkhTiqkz2dH5k2cBdGYMJYWWoups/UVAirbz3gOI69JSE6rpRIWUXqBd12nS6ts/iqO5oYk0cLAjLYToOOY8SCaA7P0BgICh80WzvzDBnGx9HbFkYI/RkhJtJLhCqqOF67KAlogbDIyEXdqKb93ZynuYP5PHfAe/tpZXZ2CNzSp7Ql2HLmrRsYOmmbQM4HTAUeKCY9HqnLOrIScx6vDlSVPAmB9Oad65XgVyMMv6QS1rrR32j9nwI2IJUhSDfgykZZes2odcIO4FiKXbbj7nTfYKvIGWuYMQ5FQ3izS/90HHgPw0MWS7Xv7KAHWkinv/DSXFYuhK9HZ4un3OQEf/nTV+ttNn4cyWS5s/JoNX4PCpzHb+aoMffxyCQrCLmO5HCqojZgxwNgOzDWi+TwCG+3i/syNunV1LP1++NSC9O3I/L8To0rTms6sD8u3/A08wEChsxikbAAAAAElFTkSuQmCC">
    /// 2. `#f9a875` <img src="data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAADIAAAAPCAYAAACm25zCAAACMklEQVRIiW2WQRbEIAhDPbTcSW+oK2fRGn7oLHztdFqEJATbnv3sEWfNOHv2s+dzv2Y/a8RZ43mmd8bz357xvDvi7NG11rv26IqxeX330/s35nt/v31y6fnNzO8Xvrt5tUzkBrlFBZK5m4Rt8iQXSkxFIOGbhGIDHO2jpMOLHP58Ic7W9VlNyYKZ5z4ZWdwAiWVCXUUuiwGWxWhFOLD//e/uzQXgBGqI1bYHUMEmQraycIuaWFZwR6xkSqzf5MH+4nPlEh8mUqa5x82xSXem/0RbPTSy2JRiLSKsWKEJCeWznglL1vx9mfY+016U3OinEaEsqBttbMSlApydVROVzMAOGx3S+j6HTAkGCzTwX0akOSsIDUet42rI6neubF6X6iLyZB/MCBgzDeRqTttP8w1A54xDuWw9Z+EMnAC41XZt/O1DSHn2kuAf6dKQinuma8k2oXXS/XEONwIHwBs95ZvNzUZNhwswEA6cmRCk/cZrlIEFrHZrC5Kruq6WOvlON6bSSMA4TYRDkPOKjKjZiy1yWpo0bHgVqZTGtGShees7SnJUgLqBZaeHT5xXWgs00qOJpk9Un+Lm8Tax0dhmBhy84Qx+pApANUSd2QtcYxKJaPkYrOQ8iU9C1uDsI0qXjmeOFoUNZ7yagBT05tJSJjw+4GpzggnXWXFdjlJNVBWbtgkrtvkDx6Jr8niyhsdvqd8wJP185Ewk2n4U8YNkt0SYqFiZiTq1vz9S7cf7scowzg/JEQahq2B2xAAAAABJRU5ErkJggg==">
    /// 3. `#eb6b6f` <img src="data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAADIAAAAPCAYAAACm25zCAAACPElEQVRIiW2WyRHDMAhFXS/MiL5REcrBAh44h4yXSAL+An5c7Wy142rHdR0XO1t4Xcd11Rp577fG1XLt1net6zpb7D7b2WL3nLs/1ko8W+591971cvcq7u+vcnj3PX4X7rtwM9BN/g26amMEjr16iwYYlczKZF0XEmThlmdlcYiV+7TnSuAfLtqRyL3fSHhLoekt6ZWFkk2Xjv4GIAHODna5PoqTir+DARmMA/jHEThQ2EG38BDrjDBRqT0eAeNZwOiQBZNNFgTsxnmKfX+AcrXzZPDwQR4OPcInm0VoZymB0IXkv+yFLBgzE28FDZ81ny4AaVdaGmbkAV2HjuQKJfpp+EdRAKRU9z0hh8QILiVPc7empOs87EYNtYYiTQY/4P/y12CKLA+DlpytscBulnnpeE+Jq50nNf0XAWsUlgToI3Ysq6KIcjwLGEILTtOj42URKWvkwHyCEUcyNHeZa8wOGnC2QnY8mN5Hknker0j8Y2p0LlcqpM7POfKpUviOHWm25i6zktbQNmZKMVR72iyh1Hguh+rw5FPt9k8HSWP1ad60LT3QRqCe0JTpd5ByXc4f+iHZ6IrZ0X47VdNwfUCW1snQaL3ZOGabxXwa8msFgY3W3f79d+M9PJjINNrnBP4MSKCf6LGN0vxICMj3rwV+ssB76HZtNomdp2mXjJAh6vUjAZq2zwCiTuTrs6UPU0dh1RExmCHnHnNFIdYO5TwoyfRCaNz+HURWObG7T0qaBd785przhfKcnewHHJQWOWe7HyEAAAAASUVORK5CYII=">
    /// 4. `#7c3f58` <img src="data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAADIAAAAPCAYAAACm25zCAAACLklEQVRIiW2WQXbEMAhDfTL7GvZd8NnTRQL6YrrIm3bsgBBCzLjrPLHOc+d57tpPTP39Pud95v7u7ff8+4z53om5n1j7/R7v5JNn8cWN+eaKlWf59/kw8LuMuy0P3xthIBGsAhIkk2VB5wN0KngCrLP5Fqa424FmUY2gqHhfrvXFrVgibeSFYnKqE0oA0BVoF6BY6GAVJqAZm/dUKD+3kUEiVMxGDBE7oieYAnOnWNT5qSAlOXTukl3INu8n67zLONEJXCrQOoi4Mc8zag6g2WQsLwUSVoDqIOWDd0nCFBnFYs2iik7QLlV2QwTnnOa9EajqGpMAusRoLzq1nEA0wJlYOg6TBM6tKBAB5qmYIgJzrWG3tqr19jKlhK7ZfGDeOAfRmW7uU4UtJy5WK4z5gWO41uk0Gi52yYb9P6cziwZoyKMKrW5vY9dAQ97d+Uj8IEsdeAavznCekNxMogB2y6QJ9J3kMnGJgoDMs7b/n8PuOvS9UN+V3ptLWcAmPy5X7ASTZGP7snNUSrkXnO/XfpsD2SZtRkCWf6x0t8TuNCp2A8hpd51xw9G6SJcdwTbZ4y5GJrzFciIrFh2iw90Wn12LBp6zGNZRSvh8hRggH1620zsGWZTUuDu6RXLrNxmVrEXmbXfKYLBssxN5Pi4Z68zb3PigS1paiF1C/nMDZrDI7vbYBDtFmH5oclVoLIbtAQyYtZtAuofPDsSXl8uHFgsnnL9sayG2QV8gA7j+AA77EpX/BHO+AAAAAElFTkSuQmCC">
    IceCream,

    /// [Hollow](https://lospec.com/palette-list/hollow) palette
    ///
    /// 1. `#fafbf6` <img src="data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAADIAAAAPCAYAAACm25zCAAACIElEQVRIiX2WQbLkMAhDff8D21Vo4VkkwBPpP4tUVwfbgCTkLOlc6dyIfaVzFftK+4b2VewbOjfe95HPG9O7J98rY9o34lxFn6c42HvqnYJ5nn3PWfm/Y6r48zy5n9hSJhSSBJvb/YsiQihaOFhoVgAH8d7Ta7OgBm2cmYDr9H4Av3rTBuqn0H4K6WaFBgTkcm+hr4F0AlJsdJO9tpnrfAC5gGgmMr5kHe4fz0HsoEjIJteAbsqr0M9YOHN8z1pEcCnh0Zzi3GXUGmLdAJFhI8XAZ38zmUVGnceCHBw/l9IerESDl/mXgkW6PERkP2ywuF/aRoODHTeX4/nSRMguTaH+Z+xtJGfCB66Hi25VzvOrSQz1V4YAC1q33LXu63Bsvo0k8++U1nAB6M83UMc+/KZtMkCXyYKApA2uzQPmjg8Bp0rivNKyYfqfpskEtIv1dbf8hbAZi9u8WCAZNnZoLq2e5S7hiH+cDBKjISQQTjfkCLuOCQYlx/vnkwuMA+Rct2x4YXkydjZo9AtMgSZFIJjYz+Rv5RmAtpRw73zMomdrEenuctALmcxPD7v94wuK2Sx8v5qpGeOt7feX30WYN7CyGnG3S0uk87tJmoEaYRZD1My2h1WHNQSzoZWP2WOOFTGQh/tMObn7OFMcTEM4HNUqoJp2S/dvtmkuM2dLcPVA0iXcSWzwTSpE2H398/kyNN+MtAXPS9Y+OGOARcDj3H/8snmrmfmYzAAAAABJRU5ErkJggg==">
    /// 2. `#c6b7be` <img src="data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAADIAAAAPCAYAAACm25zCAAACNklEQVRIiW2WMZYDIQxD5/4Hwh10cCxSAPIXpMjbTYaxZUuW+XrUOUqbPdrsUWcvbY44nzpH1NlL3f83nT2/96g6N8r+PZri9HLi7lg7x8m7PnhmZ5lrx4t1vpemz4g6P4FkUAFsT3KBN2BowA684uxEJb8vEFUAslkt89rZu3mnoGzsKHV+g927GOgEEABBwJa4AQiZ8YITRIUSzjPgCM+3cDUxt7Cv558DpoTO9/YfgIIm3So4yGoWldJ85afcYnHHVrHeEOXeZz+BYlAkpYaPbn0OLgY1W806KSkpV1VBKTHICAUdSXU2yua5bkakUVKYCal7JQdovusAsqMEc5qgBnJuKGMzoKoGq2Fg+kuHeiln5Ul3JlDn8bsx9IdFFscZSJnAXFCg5gKNZvzvzIe5BQNxsOgu5z26CuWDd/SXtotCzC3/yI5uyXlOG5drIZhp1Yf7Gb5SvVjJhebxFucFpWRSLjlzOUOulFwHe9ht2V2De/R8a50JTYbFO2w2CsZutjuYS9DVmB923pfrWojhFLnt0pmoU0gKQ00L9wb813rm+LNnpAjE0plqDO5C4DDR/KVAgdwN2trsHlij1rEsU3rM4443UBSvSHlzeOdz7RG7z9yzAHmVLDTvWGQN88Nd9NhmSsbuYXApLlE1JrJQXk0O9q/zwUXhsI47M4/eAYK7RsVfuqd9U8qU1IDE7PoCNaT9Yg+8uwOyCHbMtzFt2Wz3KjwvkM2lYfvrtd0RVAKYgTJ+a4118UXdGpgAAAAASUVORK5CYII=">
    /// 3. `#565a75` <img src="data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAADIAAAAPCAYAAACm25zCAAACNElEQVRIiW2WURLEIAhDPaXO6DXk/O5HBV5wP3a6tRUhCaFtLDt97dPndx3Lzpj3/q59z/cZ086YdvrCb9p95jEsrmPZGet7NhBrzNzXp5/n6/e8G2f4npuXny8xp53mgSNJKSiDjZmb48dkVyloegI7Dot4XryD5ufdnwOXZ28AZgLYuHFaVOobAjUksTKpeliiuQUAL8zRVTY3kkjGyGC84yCCpVgP9ezTOtkIJCCTcmjQ7u/KHi+WDN5CgCwBI1uiirjfzxpl6LGaFmAiqS7J7CIhyGt6cSYHjAJEFuqsukQclNKX7M+leXgPes5NkiBdRU65MVF/+0n7gj2UxRVTqLp/+o0MW2HRn99CBOl/fbCSqVfDHpxyS6ayCJNCVDZYgyz/STL+870JaVGL/jAQgwTUDk2CdblXLXcwQwNIoLKf2EdiAmCKkv/sV5KHVHANybCQAMBSTtOUAWg+EWYPFDfDjApVgPWQdCn6Y6TYawetOZAsglTnGiiGjHEgptSysdNyvSBKmAPSBMScbSZ5t3CJqn80XmoTaCAZDreYC+FGxbHgaCwipznOLn0VEuVAvPtaR/CUBqVG5EmpWmb2yes6MumLuYibQZbVDXXGoH8oLX7DaJPTTaqG1Z4DZeyt03wAKAFjalKU5tuPeVYMyG8gonn/Tk07tbHrh540I9B8eimcT99JewVIGMr6xVDlLAOxDMCwUcMhQKHMmFwjyqaMAnUxjD8KUIfCzFgKHEH5Ae5/RyEQ1jJCAAAAAElFTkSuQmCC">
    /// 4. `#0f0f1b` <img src="data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAADIAAAAPCAYAAACm25zCAAACJElEQVRIiW2WQRbEIAhDPQHs9f7n7CxU8oOz6JtOS4WQEB0Z84tcX8T68lyR88tc55r7ec4vYn0Rc9/fmPrm3Oe+jxMTtv6sdzfPvj85zvub9+aI2OvkyR/JOnbsiMBHCE58EACTFr9qoQ3yxJz/GT0hCzxAUVSeNQp0gcO9NWNW80YWWnSwFhYgshXBwpeY6kWgWF+PIMRmVC4VGIhXzFWDgI3q5pVEB3ULSAEpZs7zBxwkYoxE/0UjLP8rUcn35DclzA1EkrmLq6jqXBUlzVIiWvwCWwVO8+ZS6dIpQIFYSjrEbLQax4OyEquTN6ENKDWdyxkDWy4BMWjzUKxPY82MKMWa8mgWh3Ux22IcXLpHzUwvju7STUEzUpLrLobuuyT/XdPWH95xTxRGKYd9FYOS0HTm7BllAQWEwHdllIzxnFLinJW06uUjmWm/7vdgKsXUX7+HzGw/CYLBfDaZvhZ//+v9MLtNFJAC5/sKmGkDTiCyWciF+0Fv0p9ZDTBujtebe4GQ9kBXwjqMoQ0Vbgk5pIhXgZAHN08YgpibzhgbW3MpZkYfPLqJOtI0y90YoFXM9MJqj2kzAieijGwGk8BdUrT8YUEPZSrMdcqkTDg9BgzV+miAWTQu2q02TKrhNYDxOgmOC/B/UUvH4pC3wTZXAWMFrhsFThNUCGcIRxturnVo5P5BEDxbsVu2a5sdTmtIB1kFdkZNEVSFb8S0fJnCfv4DfOSNzUYU7UIAAAAASUVORK5CYII=">
    Hollow,

    /// [Wheat GB](https://lospec.com/palette-list/wheat-gb) palette
    ///
    /// 1. `#fffad6` <img src="data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAADIAAAAPCAYAAACm25zCAAACM0lEQVRIiW2WSxIEIQhDvf8B8S5YJbNogRedRdf0Z0QISXCEz9jLYq8Z3/2MWPnOYvv3G+d71L2Frs3/2Xfv86z91uwTe5/7jLNPjFj57luzK/6J4722rvy2LEZuGCfpqIKs3veGUwvLDVk8YkTGXYzf8QoMz+RnARoEIYG7cmvQZ4y9rDf17so+yNbGQLYKu4N6o8e1fRmSNiTZiRawbgBiotAsfvbvmjEENaBw0yav7UgmQVgXWoWkUoHobiSRQHWBuqd2X3PN4ka4/WmnaiUKJSIzu7A/qLFg0c0CCCzaCYqh+627eudvwUMEfaEnKIjoM9FGqhKWeE1XJpdxSlOkoTzDBJyAvO/Hls2hES4AlZ7WI6kCgu5TF7gN3ZGeRFvMBW6a9NVYFoMiFdTE4ijA5rrQh3Z7UWgj0UpC9IeCXAsT0yG9QMHwo5EOpu51u1MLHsK8NsnvYhzVIbsSN0mUQHQ+bQ4ya2AWLXYH30gnBmBHBMFuec8HuhfQdRO6COdp2eKO6OhlElWMW4zWwlU1qEXEAnRpG24UBUmIWQep6ixPETJXcLqobqGDOnjnN9m17SpwTvWeDXAsXBxgXRiKR8d4GqhiUJjMjjIAk2e66mCLm7ONKqewWuAURHgEIXU4U4LFl6GQ8yZd1fOf6umedUOSdU2aHVE3se4UZ8B6kbqH18b/KFYRr3d3nnMZBySkMN4JDn+nWG8fR5JFFxmQao9i7+zApSPRmuisC2qadh4/QDono9MrMmgAAAAASUVORK5CYII=">
    /// 2. `#e6c12b` <img src="data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAADIAAAAPCAYAAACm25zCAAACNUlEQVRIiW2Wy7XEMAhDXY/d/95uBsrwLCbAFcliTvKSmI8kxBu25/Wzrp/5/Na1va4/z+3E/bx25vX9f2ZxPevamc938W5e2zNjeNzjeZz3vZ78Eafe/et4vov8z3eO/H7WHZUEBTNZBMN3vQjbUcDMpqzFCaCMYJ0Oxrxf9XjWEA1Fg/F+3uFZwEKCmSh5IBQFoVmesV3MJCOJHhjJRophl9wrY8fZyBtxIwZrGUGbC11Ah4igSMrLT7Fmcl0fUgBzAYA0TVkX4hW33tnujFAGREmksVpxK7/XMyguWBMZFOp+9IyjkWwYANb5NgJn3SFUE+0cfppBJevDbBlcpeYfDPe508ZbPsYD0MXM/92oRHQJukcNa6Ii9ygKZ7JBGATnRA2DuaCAs6QOGged8i8tOlMPjHdFpVojEybtkKJTQiyYJkETadovUynQRCkPMMMaQjyonq/WS8vNBOkuuhcoPfnRZncBUfuBe4izUrmi+VHFTkEwJdWGUq6YL8rH2vPXLtkEDMaBgTZhekpszlh8lzNCTdMlahB1+aU1J72QDPZQSUgLcuTh4LJxZROMbGXNznw2e3OiGnbSryg4muH+qA2uTXe02Si1X9JRF/XWmLGuE8OOjU70xLNlQU5Bx1/0ozA0Jpse5vDa6FIT5AoLpqP5jj3SFuGbGd0fMuy0ScxAMfwefLFtDjP+5r870jCXJWIOpb1t045OaJm7pi+vo6DQ1jmc/ikXNIN9RRmxgYq77g/CuT8uJuC0cQAAAABJRU5ErkJggg==">
    /// 3. `#7a3921` <img src="data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAADIAAAAPCAYAAACm25zCAAACNUlEQVRIiW2WQRbEIAhDPYreAq6mh3cWFfjRWczrm7ZCCAm0Te97et/T+l4+9rSxp/U9fezlcT337Hse96eNvfJc3yvOnevMc1+O793zY54Tc1n8EPfkXwdjYQgc3732HRr1QgCK4AATB/O5jSRhJgAWMApM5ukJcjGG6zWAV65eBPooMg7GphVWMZOsJoMacNm/wsgmAFox+BKixUmxJ/eKuCBvoahWchp5gEyINBzM+xDGA6xKCfECVEpoFPALQ3Y0SXylXDL8cjVWV0WQlQ6QTN6zreWNA8iHJE6ASRbYRFcmChfC0NFFsqCkpsaGhCADtlmMy0L/SCh8tYyx2RXkhiL4rIoC4fTmuWpHIhg7hGmRHsB0Ixjxz+W3jAvJyjsYLOoddM6rm5x008du+SBNWklL7wDkFVzlh8BWxVBGwi58UNOnvPDG0/G+EGtZ/6R174JirbpSQSE97AIxpzMmQGFYcB9RNtxr5QMMB/iPKmlpWmpb2BhawFUgO5O7xhjvfo/y7doxLlAOB6qDUwsLs/bILSExG8bgvXOuhann/3e2FiQHBd65zmaXHMXDP9N6bHZMG0hBdazTLcerBFXdiqlFNmSZIxtnDP+R6/ncCWnxM2FdIJ4FeTHJ76aaJhjV1DuT/x0e/emG5Lm+ALi3lp3NnpMnDauL754qstQot2D3ApN74jorhTly0WsslHI0JbEVU7qZRS5+y2Y8QUV+1kUu9IJ0ULyoU6g6T3IoeUpx7B+pXFbKvu+lggAAAABJRU5ErkJggg==">
    /// 4. `#240024` <img src="data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAADIAAAAPCAYAAACm25zCAAACHklEQVRIiW2WQRbEIAhD8S5w/yPOLBTyo130TdupEJMARkX+Mmpfq3419/t9rTz3+auoX62a+/6/Y+zf/i73t0v38//q9flLxKuDoXNP3vOuzveNaT/vKwSOoNIA5klaJ3CumvcDqIMiTgMgsE5u6whwwInY6rWLWJW/Vv6imewrkWAYIqgAuOXva8CUJSXjWx0qqjUiq4GDuCWFdNUoHbdEk2TJIgoKlvB/wZIkxBW84iBG3qybAnKA2R4bPYoIfNuBH2Qz2wuvS76uYYmWMLuE8rAOzUqt8l2DC+otkHNiBmUTcGyATBK8FX6CTVjBrOksMqbylLlASrjq4yIrdnQG2sQLT4UmoF+AvmyqDc6z2SgB1r0/MaxhEAettV4GvSUTsFroyG7qqbYm8WU161K9ubFvGXn6pl5yLntG4iMCspbMTnTVjtVXCNB0wg//F9a6VTBLbL1m0lMrZ6OhuXExZF0mbUNqic4QrUObUAEvbK+HyQWbNTmDj/WJ/MEZoEnuNhkpTZ2rbrobMQ6HJ8FHmoJUgZa77f2lWueNGVDoBGrFb5ttn8vXbMEAaF3FJ/S0TRuyJADWeuxGG1MRAOM55x52Ji1aJj3MDsghZ4VKJjEj6ALWmLfht3Ybd9AaX5aomxlsSuyyM6HLobN8sX8fQzRD3lMBlbHTw4kTz+HvOg54W/W5crfQGyBj6vty0CFVeDyR/d7TgNfIfv4D4trCgj0Sv88AAAAASUVORK5CYII=">
    Wheat,

    /// [Rustic GB](https://lospec.com/palette-list/rustic-gb) palette
    ///
    /// 1. `#edb4a1` <img src="data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAADIAAAAPCAYAAACm25zCAAACVklEQVRIiXWWy5XsIAxEnX8UkIKNsoIYeAuQdEs9b9GnZwytX33kZ42212h7jr6Xtb2s7WnnWZxZj+9pfc9xvte9N9++59v2HG0v63vajXfjxsdj240l8fPeit931JAxPcep99x/oigvdKBATzp6FmjtJPAgfnYTTwziJNf7GatLfGneGgZ2YsQQvdao79x5srC8xE5XTMsL0WnNezYLEutDc0RKhsbcGStrSfQlrzVF2xFZ1oIaDLDw47hnmSwRuM9YvPU9P9Dl4zTZoKKehWJYMdgcNGm4AhHTgpJuOhlHKZtqoZWIQ4q6foheFN5QaEGcyBIh/C51fM4e55t2nEVHMwZuo4A/9VS4vj7qqOkZNJKCT305annehCGhET8kPx3WgD4CFOH+xyQ45XQpmgPi/gygC+JCLVI59HbyPGq9KqbgOmCdgUz7uZtUvJqLpIkI3UifZcOpnWrlMCE2PfpBhNyfg1NoCjULGjoRTmsB2XAb078XnoUOaL2xJ8o5PnTNJ6mkwp01CB0D1pyUI59pp2j8RcPcPxWdqAP0pMNhEN7Msd+XB8pTLp8fL2dTThcXtj97y//cHfj24aVOkwGkkDieZW1PwqbIcOqhlVGpgITyenKL+u7vv67I/rgd4/OsLl1qB5LwPcKlJxORnfIHVUDLNc4CzNcP1ZO4Y7Vj7qKgEhcmVkMxAmfFE1AVyBaLLeKbgJqvGsepbkJSyi59Xy5U5imGIrbdFbVwLNUwXKvuBRTFadov9Lob2nk1eYvLOO3eRDLfmAvFwIAwkaLVsOBbzz99ZIdVzH3kDAAAAABJRU5ErkJggg==">
    /// 2. `#a96868` <img src="data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAADIAAAAPCAYAAACm25zCAAACJklEQVRIiW2WQRbEIAhDPS7cTE/ZWVjID53FvPpaBRJimHUinp3xnLzP+8v3F8+OeO6evOt3b5+LeI7tz+dEvu+5NxX7jXlqb+3Le25H3nXFDcU/9XzfVZzFJPfjG5BFdcFKVt83Qb2FVsyDIoqoit3xk4CyY+y4gG885Gxwqm9nPOs0WqGrTQqCbmDd39HJAtXAmv3ZSXV8A+QGKWfksU6VUt78S63NTtgSaWY8wEZbd9TZ/Hb073uxT6lYt/qM5Fp5HICUtDpJeEsllxdIAxQL0nyOwJBWYh1pJKl7lI/Y97yK7Sq6+5YK58UKMJBgPSwBpcbiuih2NZmcYGEyIakdMC8pD2NBVxdZEiDpu9nHd134eTaUtImBY41OaI14PE+JwXgEVp1bxd62JyQRAkSXIVOHDFrSyTgJAEHGdoAAOCVJCUmqYi3NAriGFZkGhC4kluMrIbJul987ZhZNAsxBIS3eX9S4+jJP1xhzQf7v3z4OR9vE0Ps3b7rIP5ea7zvWn9lWcZdf9IEUrWOHbFrjrigB7BaS9KGLOzVc6jAuSJrGcNDJxb8kDNhT1ewz0Gpny22U1ouBGIPxMaFpx5ThZying3ylBWbNq2l5Y20swRyGzTY5n6JgnREgMeyMukDLHQQIyGCdRaQzIvvLAUaysWkPt7JLa4A5qUsyLlczm/RuVy3L3Cl54Nshsz5e8nQAH5u0To5JjntxYuRnwRwFLWeB/AFECj161TWNzgAAAABJRU5ErkJggg==">
    /// 3. `#764462` <img src="data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAADIAAAAPCAYAAACm25zCAAACMklEQVRIiXWWUbYDIQhDXZxsRPegy/d9WMgF53142k4VIQlh2urz7D7P8mXjLBtn2zi737V82f3cOOP/7z7Psvt8m85sm2ebx/eYft736M4Vz3EG+dQ8l93vbdtN5l5YEkhBlUAE/l28o8gZZ5cNnTffM2NfAupX7GYsU047Yk4lH7ncM80v9Aer1wLIFlDsjrSzMQCEFyBGxcwIhjZQZfFRABM3xYj7AtxxGlEIVAzUdSI/A41aaMjHmUASjrxkStCG7ncZgfUFVlaopOTXx2m5F4SaSyKYMLKW0XkTh1xDRkQZ6KOYB4yec5G0FXOJkUxtJI1DT9MhidQ7wWxmgn2SzWBmwND4UZApLlWgHr2rJTnhcOqH1HDFdUyLrvNIAa4WwPX6W+d0P9zOpBqxfPe2qNSIFqhzN4vfRKew2XOhjlhIqpdz/yRFR4z7P3tLBTc2a54P85FMlgGa8Kso2mQBJblWMAV7ZnOXIrUXq8/TlmXqNODU1JRCaD0+K+rZPolgSAbz59lrKlBWOw8dKs2TXzEt6TtNYdG2WWh4OgotSbEXUh91/E+WaMtlYteR8BjFL6+mBlXVCwnJKsuABHJMIFluBSYx/+5L7CWbJ2CUv+I1TvPUtBh+tag88Tl/lAylUAdstk84ZbjYx4xhETAPj922lUCPLvn8i6VimXSdJ3E4WRq42Rye15wiP77f+T1NjVUaGBLJMwIXpZ55bViNT9fhfBEABCW9wLL5q4ti/QGQwi/HRw8wqAAAAABJRU5ErkJggg==">
    /// 4. `#2c2137` <img src="data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAADIAAAAPCAYAAACm25zCAAACL0lEQVRIiW2WS7bDMAhDvRSY21N7/yvzG9TAFXmDnLZJzEcSomPaudP35eeyfafvu/zINW3f6e95vP/uL9932e9MxTu/T3sx3nu/GO/yfdeLtYzvMMeLm+/FmZPvjkwUAUwTZaBXZCSbr9kols1VrGpixhk7KBQxHA28Z9lggJBnD76/RhLJQJVBjcgdJCTSWxghC/OdydgW7DFeMNeKtwKwwAh1xLMCe0wU2pGbKG6hgSzaKmGy2gvxA6YreYJnBZjIPIs8YHeDOZXmCGQWOzQgJOyU9Oo5ZBUzwiJd4xVTxZA06Q1AOUvw0Li/RlRvGEBQp4eVlQlGSzZbQFlAk4x8GSDDHGj+rntxf5SkQF1Lmhr1I0gHW3WdD4N0q4XE6wMSnnNWO0ucP5wbXYMMEr8nAnV7FbOANCkxnRXMTDOJ1fLomXNFmnDZbCQtVLSJYBxQWiFmgppNUwCaZbuUEK0Y+4uOCKZkvnhO7ZfLqSUzJqJdsnAMM5ZaWi6ZwG6Y0hyA69I1MsWlmK5F9+nDSZnQJdQMuBOKxZJkaR25/P/myvHYaFugspeCEXbaZoJ/STioXJjiclyUYJSzl/JCUzo3ZSpi9d1EZH+dOxJ9bk8UxVmRTU1NZ3G6pZO5j3y40LArstitv2Wbb83z8o7pDUFY3H+DymEjiop2bwpbGHNXf11K2jSbkjlYAqBckOPr16Vx+d/V90vun7bFuQih78+eoVS8CtJnkHqPKeaz7x+yDGybsA4ruQAAAABJRU5ErkJggg==">
    Rustic,

    /// [DustByte](https://lospec.com/palette-list/dustbyte) palette
    ///
    /// 1. `#f5e9bf` <img src="data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAADIAAAAPCAYAAACm25zCAAACN0lEQVRIiW2WMRbDIAxDuf+h4Dqwm5kOCdaX0yGPvDZgW5ZkWsx+Yo2zVz8xx/M++4nZ3/X5b9/VnoFvxrNn3ff79Hx/Yjx7Yz2xMubqJ+65c5ydubx7sd5v48Za47Sb+M7DRiazbzFI/ianYpXk/X8jgCU2BRKfjaL2FBAJFgCL1QGYQGqJ1BqGSLwFZLJATGvp5Opv8cPQYleye5ngm2S+j4ypAgRcLD3Mq9XKAqiGHXo3gz73W6ONKJEdSuS17kkwCgUJBik1BcxmA2Y/LUgdtDo3rYLEZLsL4ouIdhRbKHLPBEU+VOaZpXNX14HfG7kpvXQTrxBWF8wkLoXWPzR1VqAQ0iIIGoDbQF4U1t40mtlPM1qBw4lUUoACHdYl0s5AoZNNJS0xM0kl5ZqUYUSC1s8uwDQFvW2DNiB20wU0wIIpxEpL0xHEnQWgo+rQH6dDLNp8+4gadGBHKGbntWi3E9FekK6Gge9szkB7lXqkGemaGmEgCnR59dTJP1cRf53fNJMASKIN3j/DzoEQlcrImP00oeIcJtXoYEx843+5WHGUkiC7USe+zy5qBvONOYK2zacp/H4RuV4O9G+cju7vHF5ywwGU6/Dzm4NrltZO3Y2nIyZADhxDixYNipmW5DCVCnaNqTeDW5zRtgNgnGsskTE0nxHfwBxIzlvwlQNqKRhdUHcyzirOCvG9XlpTi5hvupD2OxDFe0MC4mcwt87h1DGUINAP3eo5RaPmYqAy6YVZEmucH9bXj0Jdg6rbAAAAAElFTkSuQmCC">
    /// 2. `#aa644d` <img src="data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAADIAAAAPCAYAAACm25zCAAACOUlEQVRIiW2WQZYEIQhD64B4C70cHtJZKPBDz6JedymlmJDg59OOz3H2srOnnT2H/PoaOR+xvuzsNc6e443dZ79xf0/FYK1l9/t55/3N3/FY+62/3ncvn7tmvNubu3t8mdhkwiMD80NJLubi23rfedjaOJPIzQmIJTCR8I6EY4047DQFJ4BY43wVVGhlwotjdXpBLTcohDLRSdZGsZtVME5neGOtjb3JRP6Cxc+jZGJwKq3czIH4xmKBOhEKhAvtQllKNxiIPaLMkyErZpcpuA+ofRkxUBYBLCtswkNNZaxKDSW1KtaR2MbhOjO+dO8EBfrQPexpBJqgQBNtlE6WDRBnzWZ8sEGQlrK7yRa1tiqXnizXc6kGC7E3EafoWGalG5aCCB0acyT4A4Q4I4Sc7BdLW9YZGSMsLUNpZYDlI3RiXpFubC5Fji6UyTEZMNlj/xU+yrFkYde1qvZKbCpoFWglQXHaLwN0G9pp2qpqxcG0EzD2DCm3WvNTy1VkSycol8n4nhjt+HdcehNdCI2XTkTW2A5Sh3DaT9yqOZB06n8FSKq1SfYeUuYAdlgF6DXUprQHiSeL7yA9URfK7QjlmGPXZ4f+QZy2yUOjy2u5lm4rL60cYf5eUcpNNhDjRnv2D7u4lXpv/2mzbKJ0I0l2Mlabq7dc8q4lnVpY6D2B/o9aFYTLKHwpAGrDFeNkGuzzulJu1cwFjH5qk/9QThPAYrzxSkmBtbqGVInRAPTuVXrjbZfzcttuoP8BAA3lf1F8P28AAAAASUVORK5CYII=">
    /// 3. `#788374` <img src="data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAADIAAAAPCAYAAACm25zCAAACLklEQVRIiXWWS5LEMAhDfWhcZbb2wT2LDuhBMouuzsfGICSR4ceuH7vrzLu23XXs+pn579uu7+ddXv/erz3z3vcTJ+/nE2c+z3/v/cyMuzJ+XMdP61bmEuczH509VhaBoEz4CaqNNYCSExgBjj8F//4BygkwAF6utQJKAh25YL0jzlhM/knGc6HVQgLh3oW2bwFNoscC/SgpdrgWbFloMiBjW8ZARxAEG0gnP2j7RvLoUBSbB8QeAkKaRKGNykmzYErQkYCdeu4IRJ2VF5qoKLVc196LBb2kISvcJ7WcHYlc8B+6IWhOsEIj3gMEbXr1jXJ6VrWSvAfnSQ+tb/TKs6i3WUBKI9o6N84YpQPJSSCQ6FW9qCPzvjVGmgEUIv+i8ts0ct1X/GNlz+jO0StWWxVgbaIKALYo8xIn3iXdSGU6YNHiVMdyPFiJ4XveIU6TFnAJim2jiBdlGl1aItqvDsgxO0URi1ZdZlCdP4NoZmtxr0MqXUQvWrO9DtQ++06iaG+W9YsdgrZI87gfesDBCBTpIv+ILrmOddWpKh1VKCloyQadj3ObQWgmGV0LbYT4OZj6p4o8XC7nRH23ZIv/q3P1M6ddf9H+ALw0I7uDAhQf0f4u8uJATBrPX/MHk5sDtQzS9lnS9Ogtjr4gLFwL7YSnl6kdiNDfv+ZAGV4UKCY9HawkI9SdOWF90h/6SbHXSa2OVHEBZXL+CDEW7x/JeEng/YnRdfkf3alVGY3dPwZKIEYkpzB2AAAAAElFTkSuQmCC">
    /// 4. `#372a39` <img src="data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAADIAAAAPCAYAAACm25zCAAACMElEQVRIiW2WS5ZEIQhDXY/O5bH/hVkDFW6wBu90tV8ISbDZ8GX9Wza+Nfu3Zvdlw9cc3/l82djz1n1Z972m7/XGdXU+xr9czzXDzxn+nnHG9tz53ff6GfHk1xhs/D2H3wTmCWKODOjum+dSTfSTRHj5HBusmsgFdDJYxtE9/o8z8bUM4mTJCgRCQA2bjQeOrGQEdROtSd25AOaOIyEiHiA4quCocFQkE9gIf1n+7oHkQ7mOQDBvEpgGEsGyOhewqJ4H8kGzQmXSalekfw8K5KRxPhDyRGUklSyC1MrEhaX6F4AZlKwaQ1wjx00SPBVRYVHcXqjzpwqV63esaI66scE9mXgCqZRJxnxPnASmEdksLR0ktZD0YKJ0K2orXVA0AqrRDWk0QUnRD4wnQMjfLS6ns4S9QuCwyMkkyFloiJpjdek6pMcUkFjx1FJQV+i9z2kSPKvATSJ4FbWxxIVO6VyJpvW8S4RP2nUEyQSFfjQN3xp5D9V+QB0l/++4XpBCJEBAVOj6Nr3UHWhH1kgs2bsaETIGIzb6v0PTivPyatu+hI7oT0+zFAqXvXC++EDnxmcBm57h8vqpBmi9qGYVPelWaFOtVM1DYwonI62v2GdnqVBevHvY7cVehUY8SwEyIikIozdJ/ypN9o/I+WRq+Z6iSFle8hxWWWw7OZ+VIY9FzNXRQjevoJ8ex14Ferbq0yls7Q35bno9PnuGjoX+kHzSp74UKPgE9N/bT0znrPsBhgytFnT5F+YAAAAASUVORK5CYII=">
    DustByte,
}

impl Palette {
    /// Set the current palette
    pub fn set(&self) {
        let quadruple = match self {
            Palette::Default => [0xdef7cd, 0x86bf6b, 0x306950, 0x071821],
            Palette::Gold => [0xcfab51, 0x9d654c, 0x4d222c, 0x210b1b],
            Palette::IceCream => [0xfff6d3, 0xf9a875, 0xeb6b6f, 0x7c3f58],
            Palette::Hollow => [0xfafbf6, 0xc6b7be, 0x565a75, 0x0f0f1b],
            Palette::Wheat => [0xfffad6, 0xe6c12b, 0x7a3921, 0x240024],
            Palette::Rustic => [0xedb4a1, 0xa96868, 0x764462, 0x2c2137],
            Palette::DustByte => [0xf5e9bf, 0xaa644d, 0x788374, 0x372a39],
        };
        unsafe { *PALETTE = quadruple };
    }
}
