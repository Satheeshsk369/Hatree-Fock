import numpy as np
import scipy.special as sp
import matplotlib.pyplot as plt

x = np.linspace(-5,5,num=1000)
r = abs(x)

zeta = 1.0
psi_STO = (zeta**3/np.pi)**(0.5) * np.exp(-zeta*r)

plt.figure(figsize=(4,3))
plt.plot(x,psi_STO)

plt.show()
